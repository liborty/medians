use std::ops::{Add,Sub};
use rstats::here;
/// simple error handling
use anyhow::{Result,bail};
use indxvec::merge::{sortm};

/// Median of a &[T] slice by sorting
/// # Example
/// ```
/// use medians::naive_median;
/// let v = vec![1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
/// let res = naive_median(&v).unwrap();
/// assert_eq!(res,8_f64);
/// ```
pub fn naive_median<T>(s:&[T]) -> Result<f64> 
    where T: PartialOrd+Copy+Add<Output=T>,f64:From<T> {
    let n = s.len();
    match n {
        0 => bail!("{} empty vector!",here!()),
        1 => return Ok(f64::from(s[0])),
        2 => return Ok(f64::from(s[0]+s[1])/2.0),
        _ => {}
    } 
    let v = sortm(s,true); // expensive step!
    let mid = n/2;
    // test if n is odd
    Ok(if (n & 1) == 0 { f64::from(v[mid-1] + v[mid]) / 2.0 }
        else { f64::from(v[mid]) })  
}

fn partition<T>(set:&[T],pivot:f64) -> (Vec<T>,Vec<T>) 
    where T: PartialOrd+Copy+Sub<Output=T>,f64:From<T> {
    let mut lesser:Vec<T> = Vec::new();
    let mut greater:Vec<T> = Vec::new();
    for &st in set { 
        let s = f64::from(st);
        if s>pivot { greater.push(st) } else if s<pivot { lesser.push(st) };
    }     
    (lesser,greater)
}

/*  balance of signs against the pivot
fn balance<T>(set:&[T],pivot:f64) -> i32 
    where T: PartialOrd+Copy+Sub<Output=T>,f64:From<T> {
    set.iter().map(|&st| {
        let s = f64::from(st);
        if s>pivot { 1 } else if s<pivot { -1 } else { 0 }})
        .sum::<i32>()
}
*/

/// Fast median of &[T] slice
pub fn median<T>(set:&[T]) -> Result<f64>
    where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> { 
    let n = set.len();
    match n {
        0 => bail!("{} empty vector!",here!()),
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
    // set up initial function values for the bounds as #>= items - #<= items
    let fx2:f64 = -(n as f64); // -ve n at max
    Ok(recmedian(set,f64::from(x2),f64::from(x2+x1)/2.0,fx2)) // all shifted one up
}

/// Recursive partitioning
fn recmedian<T>(set:&[T],x1:f64,x2:f64,fx1:f64) -> f64
        where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> {
        let (sgt, slt) = partition(set,x2); 
        let balance = sgt.len() as i32 - slt.len() as i32;
        if balance == 0 { return x2 }; // termination reached
        let fx2 = balance as f64;
        let xn = (x2*fx1 - x1*fx2) / (fx1-fx2);
        if balance > 0 { 
            recmedian(&sgt,x2,xn,fx2) }
        else {
            recmedian(&slt,x2,xn,fx2) }    
    } 
