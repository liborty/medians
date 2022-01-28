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

fn partition<T>(set:&[T],pivot:f64) -> (Vec<T>,Vec<T>) 
    where T: PartialOrd+Copy+Sub<Output=T>,f64:From<T> {
    let mut lesser:Vec<T> = Vec::new();
    let mut greater:Vec<T> = Vec::new();
    for &st in set { 
        let s = f64::from(st);
        if s>pivot { greater.push(st) } else if s<pivot { lesser.push(st) };
    } 
    (greater, lesser)
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
    // find minimum and maximum
    let mut x1 = set[0]; 
    let mut x2 = x1;
    set.iter().skip(1).for_each(|&s| {
        if s < x1 { x1 = s } 
        else if s > x2 { x2 = s }; 
    }); 
    // set up initial function values for the bounds as #>= items - #<= items
    let fx1 = 2*n as i64; // balance at x1
    Ok(recmedian(set,f64::from(x1),(f64::from(x1)+f64::from(x2))/2.0,fx1)) 
}

/// Recursive partitioning
fn recmedian<T>(set:&[T],x1:f64,x2:f64,fx1:i64) -> f64
        where T: PartialOrd+Copy+Sub<Output=T>+Add<Output=T>,f64:From<T> {
        let (sgt,slt) = partition(set,x2); 
        let sgtl = sgt.len()as i64;
        let sltl = slt.len()as i64;
        let fx2 = sgtl-sltl; 
        if fx2 == 0 { return x2 }; // zero balance, termination reached
        let xn = (x2*(fx1 as f64) - x1*(fx2 as f64)) / ((fx1-fx2) as f64);
        if fx2 > 0 { 
            // dropping slt set, just counting its length into the balance
            recmedian(&sgt,x2,xn,fx2) }
        else {
            // dropping sgt set, just counting its length into the balance 
            recmedian(&slt,x2,xn,fx2) }    
    } 
