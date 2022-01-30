use std::ops::{Add,Sub};
use std::cmp::Ordering;
use anyhow::{Result,bail};
// const ACCURACY:f64 = 1e-4;

/// Helper function to copy and cast entire &[T] to Vec<f64>.
/// Like the standard .to_vec() method but also casts to f64 end type
fn tofvec<T>(set:&[T]) -> Vec<f64> where T:Copy, f64:From<T> {
    set.iter().map(|s| f64::from(*s)).collect()
}

/// median absolute differences 
/// minimised by any point within the inner pair for even sets
/// and by the median member point for odd sets
pub fn mad<T>(s: &[T], m:f64) -> f64 
    where T: Copy,f64:From<T> {
    s.iter().map(|&si| (f64::from(si) - m).abs()).sum()   
 }

/// Median of a &[T] slice by sorting
/// Works slowly but gives the exact results
/// # Example
/// ```
/// use medians::naive_median;
/// let v = vec![1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
/// let res = naive_median(&v).unwrap();
/// assert_eq!(res,8_f64);
/// ```
pub fn naive_median<T>(set:&[T]) -> Result<f64> 
    where T: Copy,f64:From<T> {
    let n = set.len();    
    if n == 0 { bail!("empty vector!"); };
    let mut s = tofvec(set); // makes an f64 mutable copy
    // test if n is even
    Ok( if (n & 1) == 0 { even_naive_median( &mut s) } 
        else { odd_naive_median(&mut s) })  
}

fn even_naive_median(s:&mut [f64]) -> f64 { 
    let mid = s.len()/2;
    if mid == 1 { return (s[0]+s[1])/2.0; }; // original length == 2
    s.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()); 
    // even functions return the mean of the two central elements 
    (s[mid-1] + s[mid]) / 2.0 
}
fn odd_naive_median(s:&mut [f64]) -> f64 {
    let mid = s.len()/2;
    if mid == 0 { return s[0]; }; // original length == 1 
    s.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    // odd functions return the value of the middle element 
    s[mid]
}

/// Iterative median based on 1D case of the modified nD
/// Weiszfeld algorithm. However, for large sets, it is 
/// slower than the naive median.
pub fn w_median<T>(set:&[T]) -> Result<f64> 
    where T: Copy,f64:From<T> {
    let n = set.len();    
    if n == 0 { bail!("empty vector!"); };
    let s = tofvec(set); // makes an f64 mutable copy
    // arithmetic mean as a starting iterative median 
    let sumx:f64 = s.iter().sum();
    let mean = sumx/(n as f64);
    // test if even or odd
    Ok( if (n & 1) == 0 { even_median(&s,mean) } 
        else { odd_median(&s,mean) }) 
}

// iterative move towards the median
fn next(s:&[f64],x:f64) -> (i64,f64) { 
    let mut recipsum = 0_f64;
    let mut sigsum = 0_i64; 
    for &si in s {
        let d = si-x;
        if d.is_normal() {
            if d > 0_f64 { recipsum += 1./d; sigsum += 1; }
            else if d < 0_f64 { recipsum += 1./-d; sigsum -= 1; }; 
        } 
    }
    (sigsum,recipsum)
}

fn odd_median(s:&[f64],mean:f64) -> f64 { 
    let n = s.len();
    if n == 1 { return s[0] };    
    let mut gm = mean;
    loop {
        let (sigs,recs) = next(s,gm);  
        if sigs.abs() < 3 { 
            break match sigs.cmp(&0_i64) {
                Ordering::Greater => nearestgt(s, gm),
                Ordering::Less => nearestlt(s, gm),
                Ordering::Equal => gm
            }
        } 
        gm += (sigs as f64)/recs; 
    }
}

fn even_median(s:&[f64],mean:f64) -> f64 { 
    let n = s.len();
    if n == 2 { return (s[0]+s[1])/2.0 };
    let mut gm = mean;
    loop {
        let (sigs,recs) = next(s,gm); 
        gm += (sigs as f64)/recs;
        if sigs.abs() < 2 { 
            let (lt,gt) = bracket(s, gm);
            break (lt+gt)/2.0 };   
    } 
}

/// Approximate median of &[T] slice by indexing
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
            let mut midset = tofvec(&v); 
            midset.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()); 
            // now the tricky even set that needs two points
            if (n & 1) == 0 && needed > 0 { res = (midset[(needed as i64 -1)as usize]+midset[needed])/2.0; break };
            res = midset[needed];
            break } 
    };
    Ok(res)
}

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

/*
fn partition<T>(set:&[T],pivot:f64) -> (Vec<T>,Vec<T>) 
    where T: PartialOrd+Copy+Sub<Output=T>,f64:From<T> {
    let n = set.len()-1;
    let mut smaller:Vec<T> = Vec::with_capacity(n);
    let mut greater:Vec<T> = Vec::with_capacity(n);
    for &st in set { 
        let s = f64::from(st);
        if s<pivot { smaller.push(st) } else if s>pivot { greater.push(st) };
    } 
    (smaller,greater)
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