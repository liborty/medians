use std::ops::{Add,Sub};
use anyhow::{Result,bail};

/// Median of a &[T] slice by sorting
/// # Example
/// ```
/// use medians::naive_median;
/// let v = vec![1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
/// let res = naive_median(&v).unwrap();
/// assert_eq!(res,8_f64);
/// ```
pub fn naive_median<T>(s:&mut[T]) -> Result<f64> 
    where T: PartialOrd+Copy+Add<Output=T>,f64:From<T> {
    let n = s.len();
    match n {
        0 => bail!("empty vector!"),
        1 => return Ok(f64::from(s[0])),
        2 => return Ok(f64::from(s[0]+s[1])/2.0),
        _ => {}
    } 
    //let v = sortm(s,true); // expensive step!
    s.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()); 
    let mid = n/2;
    // test if n is odd
    Ok(if (n & 1) == 0 { f64::from(s[mid-1] + s[mid]) / 2.0 }
        else { f64::from(s[mid]) })  
}

/// Fast median of &[T] slice
pub fn indxmedian<T>(set:&[T]) -> Result<f64>
    where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> { 
    let n = set.len();
    match n {
        0 => bail!("empty vector!"),
        1 => return Ok(f64::from(set[0])),
        2 => return Ok(f64::from(set[0]+set[1])/2.0),
        _ => {}
    }  
    // find minimum and maximum
    let mut x1 = set[0]; 
    let mut x2 = x1;
    set.iter().skip(1).for_each(|&s| {
        if s < x1 { x1 = s } 
        else if s > x2 { x2 = s }; 
    });
    let minf = f64::from(x1);
    let hashit = (n-1)as f64 / (f64::from(x2)-minf); 
    let mut freqvec = vec![0_usize;n];
    for s in set { freqvec[((f64::from(*s)-minf)*hashit).floor()as usize] += 1 }
    let mut freqsum:usize = 0;
    let mut i:usize = 0;
    while 2*freqsum < n {
        freqsum += freqvec[i];
        i += 1; 
    };
    Ok((i as f64/hashit+minf).floor())    
}

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
/*
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