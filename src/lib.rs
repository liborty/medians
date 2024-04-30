// #![warn(missing_docs)]
// #![feature(slice_swap_unchecked)]

//! Fast new algorithms for computing medians of
//! (one dimensional) vectors
//!

/// Functions for finding medians
pub mod algos;
/// Methods that implement Display and traits
pub mod implementations;

use core::cmp::Ordering;
use core::fmt::Debug;

use crate::algos::{evenmedianu64, evenmedianu8, midof3, oddmedianu64, oddmedianu8, oddmedu64};

/// Shorthand type for medians errors with message payload specialized to String
pub type Me = MedError<String>;

#[derive(Debug)]
/// custom error
pub enum MedError<T> {
    /// Non positive data dimension
    Size(T),
    /// NaN float NaN encountered
    Nan(T),
    /// Other error converted to RanError
    Other(T),
}

/// Convenience function for building MedError<String>  
/// from error kind name and payload message, which can be either &str or String
pub fn merror<T>(kind: &str, msg: impl Into<String>) -> Result<T, MedError<String>> {
    match kind {
        "size" => Err(MedError::Size(msg.into())),
        "nan" => Err(MedError::Nan(msg.into())),
        "other" => Err(MedError::Other(msg.into())),
        _ => Err(MedError::Other("Wrong error kind given to merror".into())),
    }
}

/// Enum for results of odd/even medians of complex endtypes
pub enum Medians<'a, T> {
    /// Odd sized data results in a single median
    Odd(&'a T),
    /// Even sized data results in a pair of (centered) medians
    Even((&'a T, &'a T)),
}

/// Fast medians of u8 end type by fast radix search
pub fn medianu8(s: &[u8]) -> Result<(u8, u8), Me> {
    let n = s.len();
    if n == 0 {
        merror("size", "median: zero length data")?;
    };
    if (n & 1) == 1 {
        match n {
            1 => Ok((s[0], s[0])),
            3 => {
                let indx = midof3(s, 0, 1, 2, &mut |a, b| a.cmp(b));
                Ok((s[indx], s[indx]))
            }
            _ => {
                let m = oddmedianu8(s);
                Ok((m, m))
            }
        }
    } else if n == 2 {
        Ok((s[0], s[1]))
    } else {
        let (m1, m2) = evenmedianu8(s);
        Ok((m1, m2))
    }
}

/// Fast medians of u64 end type by binary partitioning.  
/// Changes the order of the input data
pub fn medianu64(s: &mut [u64]) -> Result<Medians<u64>, Me> {
    let n = s.len();
    match n {
        0 => return merror("size", "medu: zero length data"),
        1 => return Ok(Medians::Odd(&s[0])),
        2 => return Ok(Medians::Even((&s[0], &s[1]))),
        _ => (),
    };
    if (n & 1) == 1 {
        Ok(Medians::Odd(oddmedianu64(s)))
    } else {
        Ok(Medians::Even(evenmedianu64(s)))
    }
}

/// Fast medians of u64 end type by radix search
pub fn medu64(s: &mut [u64]) -> Result<(u64, u64), Me> {
    if (s.len() & 1) == 1 {
        let byteslice = s.iter().map(|x| x.to_be_bytes()).collect::<Vec<[u8; 8]>>();
        let res = oddmedu64(&byteslice, 0, s.len() / 2);
        Ok((res, res))
    } else {
        //let (r1,r2) = evenmedu64(&byteslice,0,s.len()/2-1); Ok((r1,r2)) 
        let (&m1, &m2) = evenmedianu64(s);
        Ok((m1, m2))
    }
}

/// Fast 1D medians of floating point data, plus related methods
pub trait Medianf64 {
    /// Median of f64s, NaNs removed
    fn medf_checked(self) -> Result<f64, Me>;
    /// Median of f64s, including NaNs
    fn medf_unchecked(self) -> f64;
    /// Iterative weighted median
    fn medf_weighted(self, ws: Self, eps: f64) -> Result<f64, Me>;
    /// Zero mean/median data produced by subtracting the centre
    fn medf_zeroed(self, centre: f64) -> Vec<f64>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn medf_correlation(self, v: Self) -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn madf(self, centre: f64) -> f64;
}

/// Fast 1D generic medians, plus related methods
pub trait Median<'a, T> {
    /// Median by comparison `c`, at the end quantified to a single f64 by `q`
    fn qmedian_by(
        self,
        c: &mut impl FnMut(&T, &T) -> Ordering,
        q: impl Fn(&T) -> f64,
    ) -> Result<f64, Me>;
    /// Median of types quantifiable to u64 by `q`, at the end converted to a single f64.  
    /// For data that is already `u64`, use function `medianu64`
    fn uqmedian(self, q: impl Fn(&T) -> u64) -> Result<f64, Me>;
    /// Median by comparison `c`, returns odd/even result
    fn median_by(self, c: &mut impl FnMut(&T, &T) -> Ordering) -> Result<Medians<'a, T>, Me>;
    /// Zero mean/median data, produced by subtracting the centre
    fn zeroed(self, centre: f64, quantify: impl Fn(&T) -> f64) -> Result<Vec<f64>, Me>;
    /// Median correlation = cosine of an angle between two zero median Vecs
    fn med_correlation(
        self,
        v: Self,
        c: &mut impl FnMut(&T, &T) -> Ordering,
        q: impl Fn(&T) -> f64,
    ) -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn mad(self, centre: f64, quantify: impl Fn(&T) -> f64) -> f64;
}
