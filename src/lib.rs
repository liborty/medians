// #![warn(missing_docs)]
// use std::ops::Sub;
// use std::cmp::Ordering;
// use anyhow::{Result,bail};
use indxvec::{here,tof64,Mutsort,Vecops};

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
    let mid = s.len()/2; // midpoint (floors odd sizes)
    if (n & 1) == 0 { (f64::from(s[mid-1]) + f64::from(s[mid])) / 2.0 } // s is even
    else { f64::from(s[mid]) } // s is odd     
}

/// Exactly the same as naive_median, except uses hashsort, 
/// which is about 25% faster for >1K items.
pub fn hash_median<T>(s:&mut [T],min:f64,max:f64) -> f64
    where T: Copy+PartialOrd,f64:From<T> {
    let n = s.len();
    if n == 0 { panic!("{} empty vector!",here!()); };
    if n == 1 { return f64::from(s[0]); };
    if n == 2 { return (f64::from(s[0])+f64::from(s[1]))/2.0; }; 
    s.muthashsort(min,max); 
    let mid = s.len()/2; // midpoint (floors odd sizes)
    if (n & 1) == 0 { (f64::from(s[mid-1]) + f64::from(s[mid])) / 2.0 } // s is even
    else { f64::from(s[mid]) } // s is odd         
}

fn part(s:&[f64],pivot:f64) -> (Vec<f64>,Vec<f64>) {
    let mut ltset = Vec::new();
    let mut gtset = Vec::new();
    for &f in s { 
        if f < pivot { ltset.push(f); } else { gtset.push(f); };
    };
    (ltset,gtset)
}

/// Recursive Reducing Median
pub fn r_median<T>(set:&[T]) -> f64 
    where T: Copy+PartialOrd,f64:From<T> {
    let s = tof64(set); // makes an f64 copy
    let n = set.len();
    if n == 0 { panic!("{} empty vector!",here!()) };
    // starting pivot
    let (min,max) = s.minmaxt();
    let pivot = (min+max)/2.;
    // passing min max just to stop recomputing it
    if (n & 1) == 0 { r_med_even(&s,n/2,pivot,min,max) } 
    else { r_med_odd(&s,n/2+1,pivot,min,max) }
}

/// Reducing sets median using `minmax()` and secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to anticipated median position
fn r_med_odd(set:&[f64],need:usize,pivot:f64,setmin:f64,setmax:f64) -> f64 {  
    if need == 1 { return setmin }; 
    let n = set.len();
    if need == n { return setmax }; 

    let (ltset,gtset) = part(set,pivot);
    let ltlen = ltset.len();
    let gtlen = gtset.len();
    // println!("Need: {}, Pivot {:5.3}, partitions: {}, {}",need,pivot,ltlen,gtlen);

    match need {
    1 => ltset.mint(),
    x if x < ltlen => {
        let max = ltset.maxt();
        if setmin == max { return ltset[0] }; // all equal, done     
        let newpivot = setmin + (need as f64)*(max-setmin)/(ltlen as f64);
        r_med_odd(&ltset, need, newpivot,setmin,max) 
        },
    x if x == ltlen => ltset.maxt(),
    x if x == ltlen+1 => gtset.mint(),
    x if x == n => gtset.maxt(),  
    _ => { // need > ltlen
        let newneed = need - ltlen;
        let min = gtset.mint(); 
        if min == setmax { return gtset[0] }; // all equal, done
        let newpivot = min + (setmax-min)*(newneed as f64)/(gtlen as f64);
        r_med_odd(&gtset, newneed, newpivot,min,setmax)
        }
    }
}

/// Reducing sets median using `minmax()` and secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to anticipated median position
fn r_med_even(set:&[f64],need:usize,pivot:f64,setmin:f64,setmax:f64) -> f64 {
    let n = set.len();
    let (ltset,gtset) = part(set,pivot);
    let ltlen = ltset.len();
    let gtlen = gtset.len();
    // println!("Need: {}, Pivot {}, partitions: {}, {}",need,pivot,ltlen,gtlen);
    match need {
    // 1 => ltset.mint(),
    x if x < ltlen => {
        let max = ltset.maxt();
        if setmin == max { return ltset[0] }; // all equal, done     
        let newpivot = setmin + (need as f64)*(max-setmin)/(ltlen as f64);
        r_med_even(&ltset, need, newpivot,setmin,max) 
        },
    x if x == ltlen => (ltset.maxt()+gtset.mint())/2., // at the boundary 
    x if x == n => gtset.maxt(),  
    _ => { // need > ltlen
        let newneed = need - ltlen;
        let min = gtset.mint(); 
        if min == setmax { return gtset[0] }; // all equal, done
        let newpivot = min + (newneed as f64)*(setmax-min)/(gtlen as f64);
        r_med_even(&gtset, newneed, newpivot,min,setmax)
        }
    }
}

/// Iterative move towards the median. Used by w_medians
/// Returns ( positive imbalance, number of items equal to x,
/// increment of x position towards the median )
fn next(s:&[f64],x:f64) -> (i64,i64,f64) {
    let mut recipsum = 0_f64;
    let (mut left,mut right) = (0_i64,0_i64); 
    for &si in s {
        if si < x { left += 1; recipsum += 1./(x-si); continue; };
        if si > x { right += 1; recipsum += 1./(si-x); 
        }
    }
    let balance = right-left;
    ( balance.abs(),s.len() as i64-left-right,(balance as f64)/recipsum )
}

/// Used by w_medians
fn nearestlt(set:&[f64],x:f64) -> f64 {
    let mut best = f64::MIN;
    for &s in set {
        if s > x { continue }; 
        if s > best { best = s };
    }
    best
}

/// Used by w_medians
fn nearestgt(set:&[f64],x:f64) -> f64 {
    let mut best = f64::MAX;
    for &s in set {
        if s < x { continue }; 
        if s < best { best = s };
    }
    best
}

/// Iterative median based on the heavily modified 1D case
/// of the modified nD Weiszfeld algorithm.
pub fn w_median<T>(set:&[T]) -> f64
    where T: Copy,f64:From<T> {
    let n = set.len();
    match n {
        0 => panic!("{} empty vector!",here!()),
        1 => return f64::from(set[0]),
        2 => return f64::from(set[0])+f64::from(set[1])/2.0,
        _ => {}
    };
    let s = tof64(set); // makes an f64 copy
    // arithmetic mean as a starting iterative median 
    let sumx:f64 = s.iter().sum();
    let mean = sumx/(n as f64); 
    if (n & 1) == 0 { even_w_median(&s,mean) } 
    else { odd_w_median(&s,mean) }
}

fn odd_w_median(s:&[f64],m:f64) -> f64 {
    let mut gm = m; 
    let mut lastsig = 0_i64;
    loop {
        let (sigs,eqs,dx) = next(s,gm);  
        // println!("{} {} {} {}",sigs,eqs,gm,dx);
        // in the midst of the central equal items, return old gm
        if sigs < eqs { return gm }; 
        gm += dx; // update gm
        if (sigs < lastsig) && (sigs >= 3) { // normal converging iteration
            lastsig = sigs;    
            continue; 
        };
        // not converging much or near the centre already, 
        // find manually the nearest item in the dx direction
        if dx > 0. { gm = nearestgt(s, gm); }
        else if dx < 0. { gm = nearestlt(s, gm); };
        if sigs < 3 { return gm;  }; // at the centre, return it
        lastsig = sigs; // otherwise continue with this new value
    }
}

fn even_w_median(s:&[f64],m:f64) -> f64 {
    let mut gm = m; 
    let mut lastsig = 0_i64;
    loop {
        let (sigs,eqs,dx) = next(s,gm);  
        // println!("{} {} {} {}",sigs,eqs,gm,dx);
        // in the midst of the central equal items, return old gm
        if sigs < eqs { return gm }; 
        gm += dx; // update gm
        if (sigs < lastsig) && (sigs >= 2) { // normal converging iteration
            lastsig = sigs;    
            continue; 
        };
        // not converging much or near the centre already, 
        // find manually the nearest item in the dx direction
        if sigs < 2 { return  (nearestgt(s, gm) + nearestlt(s, gm))/2.;  }; // at the centre, return it
        lastsig = sigs; // otherwise continue with
        if dx > 0. { gm = nearestgt(s, gm); }
        else if dx < 0. { gm = nearestlt(s, gm); };
    }
}
/*
/// Median based on hash division
pub fn hashmed<T>(set:&[T],min:f64,max:f64) -> f64 
    where T: Copy+PartialOrd,f64:From<T> {
    let n = set.len();
    if n == 0 { panic!("{} empty vector!",here!()) };
    let s = tof64(set); // makes a f64 copy 
    if (n & 1) == 0 { 0. } 
        else { hashmed_odd(&s,n/2,min,max) }
}   

/// Does the work for `hashmed`   
fn hashmed_odd(s: &[f64], need:usize, min:f64, max:f64) -> f64 { 
    // Recursion termination condition
    let n = s.len();
    if n == 1 { return s[0]; }; // no sorting needed 
    // hash is a precomputed factor, s.t. ((x-min)*hash).floor() subscripts will be in [0,n]
    // this is then reduced to [0,n-1] 
    let hash = n as f64 / (max-min); 
    let mut buckets:Vec<Vec<f64>> = Vec::new();

    // group data items into buckets, subscripted by the data hash values
    for xi in s {
        let mut hashsub = (hash*(xi-min)).floor() as usize; 
        if hashsub == n { hashsub -= 1; }; 
        buckets[hashsub].push(*xi);  
    };
    let mut cumlen = 0;     // cummulative length of buckets
    for bucket in buckets.iter() { 
        let blen = bucket.len(); // size of the current bucket 
        cumlen += blen;
        // find the bucket that straddles need: cumlen >= need
        if cumlen < need { continue; };
        // needhere is subscript to median in this bucket;
        let needhere = need+blen-cumlen;
        // up to two items in a bucket can be done immediately
        // println!("bucket items: {}",blen);
        match blen {
            1 => { return bucket[0]; }, 
            2 => {  
                if needhere == 0 { 
                    if bucket[0] > bucket[1] { return bucket[1]; } else { return bucket[0]; } };
                if bucket[0] > bucket[1] { return bucket[0]; } else { return bucket[1]; }; },
            _ => {
                let mx = bucket.minmax();
                if mx.min == mx.max { return bucket[0]; } // all are equal, quick result
                if needhere == 0 { }
                else {  // not all the same
                    bucket.mutsorttwo(0,mx.minindex); // swap min to the front
                    bucket.mutsorttwo(mx.maxindex,blen-1); // and swap max to the end
                    // recurse to process the rest, within the new reduced range
                    return hashmed_odd(bucket,needhere,mx.min,mx.max); 
                };
            }
        } // end of match (this bucket) but there may be more
    }; // end of for (all buckets)
    panic!("{} should not drop to here",here!());
    0.0
} 
*/