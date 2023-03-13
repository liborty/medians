use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use crate::Me;
use ran::Re;

#[derive(Debug)]
/// custom error
pub enum MedError<T> {
    /// Non positive data dimension
    Size(T),
    /// Other error converted to RanError
    Other(T),
}

impl<T> Error for MedError<T> where T: Sized + Debug + Display {}

impl<T> Display for MedError<T>
where
    T: Sized + Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MedError::Size(s) => write!(f, "Size of data must be positive: {s}"),
            //<MedError<T> as std::convert::Into<T>>::into(s)),
            MedError::Other(s) => write!(f, "Converted from: {s}"),
        }
    }
}

/// Automatically converting RanError<String> to MedError::OtherError<String>
impl From<Re> for Me {
    fn from(e: Re) -> Self {
        MedError::Other(format!("RanError: {e}"))
    }
}

/// Convenience function for building RanError<String>  
/// from error kind name and payload message, which can be either &str or String
pub fn merror(kind: &str, msg: impl Into<String>) -> Me {
    match kind {
        "size" => MedError::Size(msg.into()), 
        "other" => MedError::Other(msg.into()),
        _ => MedError::Other("Wrong error kind given to merror".into())
    }
}
