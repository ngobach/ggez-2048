use core::result::Result as CoreResult;
use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new<T: AsRef<str>>(s: T) -> Self {
        Self {
            message: s.as_ref().to_string(),
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

pub fn map_to_error<T: StdError>(e: T) -> Error {
    Error::from_error(e)
}
