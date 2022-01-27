use core::ops::Add;
use rstats::{here,MinMax};
/// simple error handling
use anyhow::{Result,bail};
use indxvec::merge::{sortm,minmax};

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
        0 => bail!("{} can not take median of zero length vector!",here!()),
        1 => return Ok(f64::from(s[0])),
        2 => return Ok(f64::from(s[0]+s[1])/2.0),
        _ => {}
    } 
    let v = sortm(s,true);
    let mid = n/2;
    // test if n is even or odd
    Ok(if (n & 1) == 0 { f64::from(v[mid-1] + v[mid]) / 2.0 }
        else { f64::from(v[mid]) })  
}

/// Fast median only of a &[T] slice by partitioning
    pub fn newmedian<T>(s:&[T]) -> Result<f64>
        where T: PartialOrd+Copy,f64:From<T> {
        // define balance of signs closure
        let balance = |x:f64| s.iter().map(|&s| (f64::from(s)-x).signum()).sum::<f64>();
        let MinMax{min,minindex,max,maxindex} = minmax(s);
        Ok(1.)
    }

 
