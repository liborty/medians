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
use crate::algos::{oddmedianu8,evenmedianu8,oddmedianu64,evenmedianu64};

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
pub fn merror<T>(kind: &str, msg: impl Into<String>) -> Result<T,MedError<String>> {
    match kind {
        "size" => Err(MedError::Size(msg.into())),
        "nan" => Err(MedError::Nan(msg.into())), 
        "other" => Err(MedError::Other(msg.into())),
        _ => Err(MedError::Other("Wrong error kind given to merror".into()))
    }
}

/// Enum for results of odd/even medians of complex endtypes
pub enum Medians<'a, T> {
    /// Odd sized data results in a single median
    Odd(&'a T),
    /// Even sized data results in a pair of (centered) medians
    Even((&'a T, &'a T)),
}

/// Enum for results of odd/even medians with simple numeric endtypes.
/// Convert to a single f64 by applying `.into()`
pub enum ConstMedians<T> {
    /// Odd sized data results in a single median
    Odd(T),
    /// Even sized data results in a pair of (centered) medians
    Even((T,T))
}

/// Fast medians of u8 end type by fast radix search
pub fn medianu8(s: &[u8]) -> Result<ConstMedians<u8>, Me> {
    let n = s.len();
    match n {
        0 => return merror("size", "median: zero length data")?,
        1 => return Ok(ConstMedians::Odd(s[0])),
        2 => return Ok(ConstMedians::Even((s[0],s[1]))),
        _ => (),
    };
    if (n & 1) == 1 {
        Ok(ConstMedians::Odd(oddmedianu8(s)))
    } else {
        Ok(ConstMedians::Even(evenmedianu8(s)))
    }
}

/// Fast medians of u64 end type by binary partitioning.  
/// Changes the order of the input data
pub fn medianu64(s: &mut [u64]) -> Result<ConstMedians<u64>, Me> {
    let n = s.len();
    match n {
        0 => return merror("size", "medu: zero length data"),
        1 => return Ok( ConstMedians::Odd(s[0]) ),
        2 => return Ok( ConstMedians::Even((s[0], s[1])) ),
        _ => (),
    };
    if (n & 1) == 1 { Ok( ConstMedians::Odd(oddmedianu64(s)) ) }
    else { Ok(ConstMedians::Even(evenmedianu64(s))) }
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
