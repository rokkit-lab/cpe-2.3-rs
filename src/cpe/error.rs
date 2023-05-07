//! A submodule providing errors and error handling for the CPE module.

use std::fmt;

#[derive(Debug)]
/// An enumeration containing all CPE errors.
pub enum CpeError {
    /// The requested action contained an invalid value. 
    InvalidValueError,
}

impl std::error::Error for CpeError {}

impl fmt::Display for CpeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpeError::InvalidValueError => write!(f, "An invalid value was found"),
        }
    }
}

