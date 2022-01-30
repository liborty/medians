use std::ops::{Add,Sub};
//use std::fmt::Write;
use anyhow::{Result,bail};
// const ACCURACY:f64 = 1e-4;

/// Helper function to copy and cast entire &[T] to Vec<f64>.
/// Like the standard .to_vec() method but also casts to f64 end type
fn tofvec<T>(set:&[T]) -> Vec<f64> where T:Copy, f64:From<T> {
    set.iter().map(|s| f64::from(*s)).collect()
}

/// median absolute differences for comparisons
/// the smaller value the better
pub fn mad(s: &[f64], m:f64) -> f64 {
    s.iter().map(|&si| (si-m).abs()).sum()
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
/// Weiszfeld algorithm. Converges quickly to the inner radius
/// but the problem remains how to get the final exact values.
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
            if d > 0. {
            recipsum += 1./d;
            sigsum += 1;
            } else {
            recipsum += 1./-d;
            sigsum -= 1;
            } 
        } 
    }
    (sigsum,recipsum)
}

/// Approximate median error for testing
/// (absolute value of)
pub fn mederror(s:&[f64],x:f64) -> f64 {
    let (sn,rec) = next(s,x);
    (sn as f64/rec).abs()
}  

fn odd_median(s:&[f64],mean:f64) -> f64 { 
    let n = s.len();
    if n == 1 { return s[0] };    
    let mut gm = mean;
    loop {
        let (sigs,recs) = next(s,gm);  
        if sigs.abs() < 3 {
            if sigs > 0 { break nearestgt(&s, gm) }
            else if sigs < 0 { break nearestlt(&s, gm) }
                   else { break gm }   // exact hit ? 
        }; 
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
        if sigs.abs() < 2 { break gm };   
    } 
}

/// Approximate median of &[T] slice by indexing
/// Apply .floor() to the result for integer end types
pub fn i_median<T>(set:&[T]) -> Result<f64>
    where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> { 
    let n = set.len();
    match n {
        0 => bail!("empty vector!"),
        1 => return Ok(f64::from(set[0])),
        2 => return Ok(f64::from(set[0]+set[1])/2.0),
        _ => {}
    } 
    let s = tofvec(set); // makes an f64 mutable copy
    // find minimum and maximum
    let mut x1 = s[0]; 
    let mut x2 = x1;
    s.iter().skip(1).for_each(|&si| {
        if si < x1 { x1 = si } 
        else if si > x2 { x2 = si }; 
    });
    // linear transformation from [min,max] data values to [0,n-1] indices
    // by precomputed scale factor hashf
    let hashf = (n-1) as f64 / (x2-x1); 
    // histogram (probability density function)
    let mut freqvec = vec![0_usize;n];
    // count items in ech equal bucket of values
    for si in &s { freqvec[((si-x1)*hashf).floor()as usize] += 1 }
     // find index just after the midpoint of cpdf
    let mut freqsum = 0.;
    let mut indx = 0;
    let nhalf = n as f64/2.0;
    for (i,f) in freqvec.iter().enumerate() {
        freqsum += *f as f64;
        if freqsum > nhalf { indx = i; break } 
    }; 
    // how far have we overshot as a proportion of this (middle) bucket?
    // this is just an interpolation, hence the residual errors
    // let fx1 = (freqsum - nhalf) as f64;
    // let fx0 = freqvec[indx] as f64 - fx1;
    // let denom = freqvec[indx] as f64 - 2.0*fx1;
    //if denom.is_normal() {
    //    Ok( minf + (indx as f64)/hashit + fx0/denom/hashit ) }

    // transformed back to the original range
    // approximate but fast
    // let approxmed = x1 + (indx as f64)/hashf; // -(freqsum-nhalf)/(freqvec[indx]as f64))

    Ok( if freqvec[indx] < 2 { nearestlt(&s,x1 + (indx as f64)/hashf) } 
    else { x1 + ((indx as f64)-0.5)/hashf } )
//    if freqvec[indx] == 2 { }
//   }
//    Ok ( 
//        if (n & 1) == 0 { 
//            let (lt,gt) = nearestbracket(&s,approxmed);
//            (lt+gt)/2.0 
//        } // even median
//        else {  } )
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

fn nearestbracket(set:&[f64],x:f64) -> (f64,f64) {
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