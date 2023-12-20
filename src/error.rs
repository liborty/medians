use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};

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

impl<T> Error for MedError<T> where T: Sized + Debug + Display {}

impl<T> Display for MedError<T>
where
    T: Sized + Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MedError::Size(s) => write!(f, "Size of data must be positive: {s}"),            
            MedError::Nan(s) => write!(f, "Floats must not include NaNs: {s}"), 
            MedError::Other(s) => write!(f, "Converted from: {s}"),
        }
    }
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
