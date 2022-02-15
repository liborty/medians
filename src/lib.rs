use std::ops::Sub;
use std::cmp::Ordering;
// use anyhow::{Result,bail};
use indxvec::{here,tof64,merge::{hashsort}};

fn nearestlt(set:&[f64],x:f64) -> f64 {
    let mut best = f64::MIN;
    for &s in set {
        if s >= x { continue };
        if s > best { best = s };
    }
    best
}

fn nearestgt(set:&[f64],x:f64) -> f64 {
    let mut best = f64::MAX;
    for &s in set {
        if s <= x { continue };
        if s < best { best = s };
    }
    best
}

fn bracket(set:&[f64],x:f64) -> (f64,f64) {
    let mut bestlt = f64::MIN;
    let mut bestgt = f64::MAX;
    for &s in set {
        if s > x {
            if s < bestgt { bestgt = s };
            continue;
        };
        if s > bestlt && s<x { bestlt = s };
    }
    (bestlt,bestgt)
}

/// Median of a &[T] slice by sorting
/// Works slowly but gives exact results
/// Sorts its mutable slice argument as a side effect
/// # Example
/// ```
/// use medians::naive_median;
/// let mut v = vec![1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
/// let res = naive_median(&mut v);
/// assert_eq!(res,8_f64);
/// ```
pub fn naive_median<T>(s:&mut [T]) -> f64
    where T: Copy+PartialOrd,f64:From<T> {
    let n = s.len();
    if n == 0 { panic!("{} empty vector!",here!()); };
    if n == 1 { return f64::from(s[0]); };
    if n == 2 { return (f64::from(s[0])+f64::from(s[1]))/2.0; }; 
    s.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()); // fastest Rust sort
    let mid = s.len()/2; // midpoint
    if (n & 1) == 0 { (f64::from(s[mid-1]) + f64::from(s[mid])) / 2.0 } // s is even
    else { f64::from(s[mid]) } // s is odd     
}

/*/
pub fn hash_medianu8(s:&[u8]) -> f64 {
    let n = s.len();
    if n == 0 { panic!("{} empty vector!",here!()); };
    if n == 1 { return f64::from(s[0]); };
    if n == 2 { return (f64::from(s[0])+f64::from(s[1]))/2.0; };
    let mut hist = vec![0_usize,256];
    for &si in s { hist[si as usize] += 1 }; 
    let mid = s.len()/2; // midpoint
    let mut totcount = 0_usize;
    for (i,&freq) in hist.iter().enumerate() { 
        totcount += freq;
        if totcount < mid-1 { continue; };
        // in the middle section now
        if (n & 1) == 0 {
            if totcount == mid-1 {  }

        
    }
    if (n & 1) == 0 { (f64::from(s[mid-1]) + f64::from(s[mid])) / 2.0 } // s is even
    else { f64::from(s[mid]) } // s is odd     
}
*/

pub fn hash_median<T>(set:&[T],min:f64,max:f64) -> f64
    where T: Copy+PartialOrd+Sub<Output=T>, f64:From<T> {
    let n = set.len();
    if n == 0 { panic!("{} empty vector!",here!()); };
    // test if n is even
    if (n & 1) == 0 { even_hash_median(set,min,max) }
        else { odd_hash_median(set,min,max) }
}

fn even_hash_median<T>(s:&[T],min:f64,max:f64) -> f64 
   where T: Copy+PartialOrd+Sub<Output=T>,f64:From<T> {
    let mid = s.len()/2;
    if mid == 1 { return (f64::from(s[0])+f64::from(s[1]))/2.0; }; // original length == 2
    let indx = hashsort(s,min,max);
    // even functions return the mean of the two central elements
    (f64::from(s[indx[mid-1]])+f64::from(s[indx[mid]])) / 2.0
}

fn odd_hash_median<T>(s:&[T],min:f64,max:f64) -> f64 
   where T: Copy+PartialOrd+Sub<Output=T>, f64:From<T> {
    let mid = s.len()/2;
    if mid == 0 { return f64::from(s[0]); }; 
    let ss = hashsort(s,min,max); 
    // odd functions return the value of the middle element
    f64::from(s[ss[mid]])
}


/// Iterative median based on the modified 1D case
/// of the modified nD Weiszfeld algorithm.
/// Now also combined with partitioning.
pub fn new_median<T>(set:&[T]) -> f64
    where T: Copy,f64:From<T> {
    let n = set.len();
    if n == 0 { panic!("{} empty vector!",here!()); };
    let mut s = tof64(set); // makes an f64 mutable copy
    // arithmetic mean as a starting iterative median
    // let sumx:f64 = s.iter().sum();
    // let mean = sumx/(n as f64);
    // test if even or odd
    if (n & 1) == 0 { 
        // use the last point to start and make the rest odd
        let point = s.swap_remove(n-1);
        neweven_w_median(&s,mean)        
        }
        else { newodd_w_median(&s,mean) }
}

// iterative move towards the median
fn newnext(s:&[f64],x:f64) -> (i64,f64) {
    let mut recipsum = 0_f64;
    let mut sigsum = 0_i64;
    for &si in s {
        let d = si-x;
        if d.is_normal() {
            if d > 0_f64 { recipsum += 1./d; sigsum += 1; }
            else if d < 0_f64 { recipsum -= 1./d; sigsum -= 1; };
        }
    }
    recipsum = (sigsum as f64)/recipsum;  
    (sigsum,recipsum)
}

fn newodd_w_median(s:&[f64],mean:f64) -> f64 {
    let n = s.len();
    if n == 1 { return s[0] };
    let mut gm = mean;
    loop {
        let (sigs,dx) = next(s,gm); 
        if sigs.abs() < 3 || dx.abs() < 1e-5 {
            break match sigs.cmp(&0_i64) {
                Ordering::Greater => nearestgt(s, gm),
                Ordering::Less => nearestlt(s, gm),
                Ordering::Equal => gm,   
            }
        }
        gm += dx; 
    }
}

fn neweven_w_median(s:&[f64],mean:f64) -> f64 {
    let n = s.len();
    if n == 2 { return (s[0]+s[1])/2.0 };
    let mut gm = mean;
    loop {
        let (sigs,dx) = next(s,gm);
        if sigs.abs() < 2 || dx.abs() < 1e-5 {
            let (lt,gt) = bracket(s, gm);
            break (lt+gt)/2.0 }; 
        gm += dx;  
    }


/// Iterative median based on the modified 1D case
/// of the modified nD Weiszfeld algorithm.
/// Now also combined with partitioning.
pub fn w_median<T>(set:&[T]) -> f64
    where T: Copy,f64:From<T> {
    let n = set.len();
    if n == 0 { panic!("{} empty vector!",here!()); };
    let s = tof64(set); // makes an f64 mutable copy
    // arithmetic mean as a starting iterative median
    let sumx:f64 = s.iter().sum();
    let mean = sumx/(n as f64);
    // test if even or odd
    if (n & 1) == 0 { even_w_median(&s,mean) }
        else { odd_w_median(&s,mean) }
}

// iterative move towards the median
fn next(s:&[f64],x:f64) -> (i64,f64) {
    let mut recipsum = 0_f64;
    let mut sigsum = 0_i64;
    for &si in s {
        let d = si-x;
        if d.is_normal() {
            if d > 0_f64 { recipsum += 1./d; sigsum += 1; }
            else if d < 0_f64 { recipsum -= 1./d; sigsum -= 1; };
        }
    }
    recipsum = (sigsum as f64)/recipsum;  
    (sigsum,recipsum)
}

fn odd_w_median(s:&[f64],mean:f64) -> f64 {
    let n = s.len();
    if n == 1 { return s[0] };
    let mut gm = mean;
    loop {
        let (sigs,dx) = next(s,gm); 
        if sigs.abs() < 3 || dx.abs() < 1e-5 {
            break match sigs.cmp(&0_i64) {
                Ordering::Greater => nearestgt(s, gm),
                Ordering::Less => nearestlt(s, gm),
                Ordering::Equal => gm,   
            }
        }
        gm += dx; 
    }
}

fn even_w_median(s:&[f64],mean:f64) -> f64 {
    let n = s.len();
    if n == 2 { return (s[0]+s[1])/2.0 };
    let mut gm = mean;
    loop {
        let (sigs,dx) = next(s,gm);
        if sigs.abs() < 2 || dx.abs() < 1e-5 {
            let (lt,gt) = bracket(s, gm);
            break (lt+gt)/2.0 }; 
        gm += dx;  
    }
}

/// Balance is zero when m(edian) point is anywhere within
/// the central pair of points for an even set
/// and at the single central point for an odd set.
pub fn balance<T>(s: &[T], m:f64) -> i64
    where T: Copy,f64:From<T> {
    let mut sigsum = 0_i64;
    for &si in s {
        let d = f64::from(si)-m;
        if d.is_normal() {
            if d > 0_f64 { sigsum += 1; }
            else if d < 0_f64 { sigsum -= 1; };
        }
    }
    sigsum
}
/*
/// Median of &[T] slice by hash indexing
pub fn i_median<T>(set:&[T]) -> Result<f64>
    where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> {
    let n = set.len();
    match n {
        0 => bail!("empty vector!"),
        1 => return Ok(f64::from(set[0])),
        2 => return Ok(f64::from(set[0]+set[1])/2.0),
        _ => {}
    }
    // let s = tofvec(set); // makes an f64 mutable copy
    // find minimum and maximum
    let mut x1 = set[0];
    let mut x2 = x1;
    set.iter().skip(1).for_each(|&s| {
        if s < x1 { x1 = s }
        else if s > x2 { x2 = s };
    });
    // linear transformation from [min,max] data values to [0,n-1] indices
    // by precomputed scale factor hashf
    let hashf = (n-1) as f64 / f64::from(x2-x1);
    // histogram (probability density function)
    let mut freqvec = vec![Vec::new();n];
    // count items in ech equal bucket of values
    for &si in set { freqvec[(f64::from(si-x1)*hashf).floor()as usize].push(si) }
    // find index just after the midpoint of cpdf
    let mut freqsum = 0_usize;
    let mut res = 0_f64;
    for v in freqvec {
        freqsum += v.len();
        if 2*freqsum > n {
            let vlen = v.len();
            let needed = ((n/2)as f64 - freqsum as f64 + vlen as f64).floor()as usize;
            if vlen == 1 { res  = f64::from(v[0]); break };
            let mut midset = tof64(&v);
            midset.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            // now the tricky even set that needs two points
            if (n & 1) == 0 && needed > 0 { res = (midset[(needed as i64 -1)as usize]+midset[needed])/2.0; break };
            res = midset[needed];
            break }
    };
    Ok(res)
}

pub fn i_median<T>(set:&[T]) -> Result<f64>
    where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> {
    let n = set.len();
    if n == 0 {  bail!("empty vector!"); };
    // find minimum and maximum
    let (x1,x2) = minmaxt(set);
    // linear transformation from [min,max] data values to [0,n-1] indices
    // by precomputed scale factor hashf
    let hashf = (n-1) as f64/f64::from(x2-x1);
    // histogram (probability density function)
    let mut freqvec:Vec<Vec<T>> = vec![Vec::new();n];
    // store items in ech equal bucket of values
    for &si in set { freqvec[((f64::from(si)-f64::from(x1))*hashf).floor()as usize].push(si) };

    // find index just after the midpoint of cpdf
    let mut freqsum = 0_usize;
    let mut res = 0_f64;
    for (i,v) in freqvec.iter().enumerate() {
        let vlen = v.len();
        if vlen == 0 { continue; };
        freqsum += vlen; // cummulate
        let freqsum2 = 2*freqsum;
        if freqsum2 < n { continue; }; // not at midpoint, yet
        if freqsum2 == n { // even set midpoint
            let mut nextnonzerov = i+1;
            while freqvec[nextnonzerov].is_empty() { nextnonzerov += 1; };
            res = f64::from(maxt(v) + mint(&freqvec[nextnonzerov]))/2_f64;
            break;
        };
        // past the midpoint now
        if vlen == 1 { res = f64::from(v[0]); break }; // odd set midpoint
        if (n & 1) == 0 { res  = f64::from(v[0]+v[1])/2.0; break; }
        else { res = f64::from(v[0]); break; };
    };
    Ok(res)
}

/// Median of &[T] slice by indexing
pub fn i_median<T>(set:&[T]) -> Result<f64>
    where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> {
    let n = set.len() as i64;
    if n == 0 { bail!("empty vector!"); };
    Ok( if (n & 1) == 0 { even_i_median(&s,n/2) }
        else { odd_i_median(&s,n/2) })
}

fn odd_i_median(&set:&[T],pos:i64) -> f64 {
    // find minimum and maximum
    let n = set.len();
    if n == 1 { return( f64::from(v[0])); }; // single item, simple termination
    if pos = 0 { return( f64::from(mint(v[0]))) }; // first position is the mipoint - need the minimum value in this bucket
    if pos == n-1 { return( f64::from(maxt(v[n-1]))) }; // last position is the mipoint - need the maximum value in this bucket
    if n < 6 {
        s.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    // odd functions return the value of the middle element
    s[mid]
    }
    let mut x1 = set[0];
    let mut x2 = x1;
    set.iter().skip(1).for_each(|&s| {
        if s < x1 { x1 = s }
        else if s > x2 { x2 = s };
    });
    // linear transformation from [min,max] data values to [0,n-1] indices
    // by precomputed scale factor hashf
    let hashf = n / f64::from(x2-x1) - f64::MIN_POSITIVE;
    // histogram (probability density function)
    let mut freqvec = vec![Vec::new();n];
    // sort items into each equal bucket of values
    for &si in set { freqvec[(f64::from(si-x1)*hashf).floor()as usize].push(si) }

    // find index just after the pos portion of cpdf
    let mut freqsum = 0_i64;
    let mut res = 0_f64;
    let mut saved = 0_f64;
    let ni = n as i64;

    for v in freqvec {
        let vlen = v.len() as i64;
        freqsum += vlen;
        let midpoint = (ni/2) - freqsum + vlen;

        if endpos < 0 { continue; } // not past the midpoint yet, can discard these buckets

        if (n & 1) == 0 { // even set (more difficult, as two bracketing points are needed)
            if endpos == 0 { // the midpoint lies between this and some following bucket!
                saved =  f64::from(maxt(&v)); // save the max element as lower bracket
                continue; };
            if vlen == 1 { res  = (saved + f64::from(v[0]))/2.0; break }; // the upper bracket
            if midpoint == 0 { // several items in this bucket but midpoint is before them all
               res  = (saved + f64::from(mint(&v)))/2.0; // so use their minimum as the upper bracket
               break;
            };
            // two items and the midpoint must now be in between
            if vlen == 2 { res = (f64::from(v[0]) + f64::from(v[1]))/2.0; break };
            // any number of items - sort and bracket midpoint
            // can not simply recurse because midpoint is not necessarily in the middle of v
            let mut midset = tofvec(&v);
            midset.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            res = (midset[(midpoint-1)as usize]+midset[(midpoint)as usize])/2.0;
            break;
            };
        // odd set
        if vlen == 1 { res  = f64::from(v[0]); break }; // the only item must be the midpoint
        if endpos == 1 { res = f64::from(maxt(&v)); } // the last item is the midpoint

            let mut midset = tofvec(&v);
            midset.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            // now the tricky even set that needs two points
            if (n & 1) == 0 && needed > 0 {
                res = (midset[(needed as i64 -1)as usize]+midset[needed])/2.0;
                break
            };
            res = midset[needed];
            break }
    };
    Ok(res)
}

fn even_i_median(&set:&[T],pos:i64) -> f64 {
    // find minimum and maximum
    let n = set.len() as f64;
    // only two items, the midpoint must now be in between
            if vlen == 2 { res = (f64::from(v[0]) + f64::from(v[1]))/2.0; break };
    let mut x1 = set[0];
    let mut x2 = x1;
    set.iter().skip(1).for_each(|&s| {
        if s < x1 { x1 = s }
        else if s > x2 { x2 = s };
    });
    // linear transformation from [min,max] data values to [0,n-1] indices
    // by precomputed scale factor hashf
    let hashf = n / f64::from(x2-x1) - f64::MIN_POSITIVE;
    // histogram (probability density function)
    let mut freqvec = vec![Vec::new();n];
    // sort items into each equal bucket of values
    for &si in set { freqvec[(f64::from(si-x1)*hashf).floor()as usize].push(si) }

    // find index just after the pos portion of cpdf
    let mut freqsum = 0_i64;
    let mut res = 0_f64;
    let mut saved = 0_f64;
    let ni = n as i64;

    for v in freqvec {
        let vlen = v.len() as i64;
        freqsum += vlen;
        let midpoint = (ni/2) - freqsum + vlen;

        if endpos < 0 { continue; } // not past the midpoint yet, can discard these buckets

        if (n & 1) == 0 { // even set (more difficult, as two bracketing points are needed)
            if endpos == 0 { // the midpoint lies between this and some following bucket!
                saved =  f64::from(maxt(&v)); // save the max element as lower bracket
                continue; };
            if vlen == 1 { res  = (saved + f64::from(v[0]))/2.0; break }; // the upper bracket
            if midpoint == 0 { // several items in this bucket but midpoint is before them all
               res  = (saved + f64::from(mint(&v)))/2.0; // so use their minimum as the upper bracket
               break;
            };
            // two items and the midpoint must now be in between
            if vlen == 2 { res = (f64::from(v[0]) + f64::from(v[1]))/2.0; break };
            // any number of items - sort and bracket midpoint
            // can not simply recurse because midpoint is not necessarily in the middle of v
            let mut midset = tofvec(&v);
            midset.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            res = (midset[(midpoint-1)as usize]+midset[(midpoint)as usize])/2.0;
            break;
            };
        // odd set
        if vlen == 1 { res  = f64::from(v[0]); break }; // the only item must be the midpoint
        if endpos == 1 { res = f64::from(maxt(&v)); } // the last item is the midpoint

            let mut midset = tofvec(&v);
            midset.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            // now the tricky even set that needs two points
            if (n & 1) == 0 && needed > 0 {
                res = (midset[(needed as i64 -1)as usize]+midset[needed])/2.0;
                break
            };
            res = midset[needed];
            break }
    };
    Ok(res)
}

/// Fast median of &[T] slice
pub fn median<T>(set:&[T]) -> Result<f64>
    where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> {
    let n = set.len();
    match n {
        0 => bail!("empty vector!"),
        1 => return Ok(f64::from(set[0])),
        2 => return Ok(f64::from(set[0]+set[1])/2.0),
        _ => {}
    }
    Ok(recmedian(set,n-1,n-1))
    /*
    // find minimum and maximum
    let mut x1 = set[0];
    let mut x2 = x1;
    set.iter().skip(1).for_each(|&s| {
        if s < x1 { x1 = s }
        else if s > x2 { x2 = s };
    });
    */
    /*

    let (sgt,slt) = partition(set,set[n/2]);
    */

    // set up initial function values for the bounds as #>= items - #<= items
    //let fx1 = (n-1) as i64; // balance at x1
    //Ok(recmedian(set,f64::from(x1),(f64::from(x1)+f64::from(x2))/2.0,fx1))

}

/// Recursive partitioning
fn recmedian<T>(set:&[T],l:usize,u:usize) -> f64
        where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> {
        if set.len() == 1 { return f64::from(set[0]) }; // simple termination
        let (sgt,slt) = partition(set,x);
        let sgtl = sgt.len();
        let sltl = slt.len();
        let fx2 = sgtl-sltl;
        if fx2+fx1 == 0 { return f64::from(slt[0] }; // zero balance, termination reached
        let xn = (x2*(fx1 as f64) - x1*(fx2 as f64)) / ((fx1-fx2) as f64);
        if fx2 > 0 {
            // dropping slt set, just counting its length into the balance
            recmedian(&sgt,x2,xn,fx2) }
        else {
            // dropping sgt set, just counting its length into the balance
            recmedian(&slt,x2,xn,fx2) }
    }
*/
