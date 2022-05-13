pub mod commands;
pub(crate) mod utilities;

use std::convert::From;
use std::error::Error;
use std::fmt;

pub type AllyResult<T, E> = Result<T, AllyError<E>>;

#[derive(Debug)]
pub struct AllyError<T: Error> {
    message: String,
    source: Option<T>,
}

impl<T: Error> AllyError<T> {
    pub fn new(message: String, source: Option<T>) -> AllyError<T> {
        AllyError {
            message,
            source,
        }
    }
}

impl<T: Error> fmt::Display for AllyError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<T: Error + 'static> Error for AllyError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(source) => Some(source),
            None => None,
        }
    }
}

impl<T: Error> From<T> for AllyError<T> {
    fn from(source: T) -> Self {
        AllyError {
            message: source.to_string(),
            source: Some(source),
        }
    }
}
