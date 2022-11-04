use std::fmt;
use std::error::Error;
use std::fmt::{Debug,Display};

#[derive(Debug)]
/// custom error
pub enum MedError<T> where T:Sized+Debug {
    /// Non-positive data dimension
    SizeError(T),
    /// Other error converted to RanError
    OtherError(T)
}

impl<T> Error for MedError<T> where T:Sized+Debug+Display {}

impl<T> fmt::Display for MedError<T> where T:Sized+Debug+Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {     
            MedError::SizeError(s) => write!(f,"size of data must be positive {}",s),
            MedError::OtherError(s) => write!(f,"Converted from {}",s)
        }
    }
}
