use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Error as FmtError};
use core::result::{Result as CoreResult};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new<T: AsRef<str>>(s: T) -> Self {
        Self {
            message: s.as_ref().to_string()
        }
    }

    fn from_error<T: StdError>(err: T) -> Self {
        Self::new(format!("{}", err))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> CoreResult<(), FmtError> {
        f.write_str(self.message.as_str())
    }
}

impl StdError for Error {}

pub type Result<T> = CoreResult<T, Error>;