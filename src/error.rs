use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum FakerTypeError {
    InvalidType,
}

impl Error for FakerTypeError {}

impl Display for FakerTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FakerTypeError::InvalidType => write!(f, "Invalid command"),
        }
    }
}

