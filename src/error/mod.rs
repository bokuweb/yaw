use std::io;
use std::string::FromUtf8Error;

use crate::decoder::DecodeError;
use crate::vm::RuntimeError;

#[derive(Debug, Fail)]
pub enum YawError {
    #[fail(display = "input file format error. please input wasm file")]
    InvalidFileError,

    #[fail(display = "{}", message)]
    DecodeError { error: DecodeError, message: String },

    #[fail(display = "{}", message)]
    RuntimeError {
        error: RuntimeError,
        message: String,
    },

    #[fail(display = "I/O Error: {:?}", error)]
    IOError { error: io::Error },

    #[fail(display = "decode failed: {:?}", error)]
    FromUtf8Error { error: FromUtf8Error },
}

impl From<io::Error> for YawError {
    fn from(error: io::Error) -> Self {
        YawError::IOError { error }
    }
}

impl From<FromUtf8Error> for YawError {
    fn from(error: FromUtf8Error) -> Self {
        YawError::FromUtf8Error { error }
    }
}

impl From<DecodeError> for YawError {
    fn from(error: DecodeError) -> Self {
        let message = error.to_string();
        YawError::DecodeError { error, message }
    }
}

impl From<RuntimeError> for YawError {
    fn from(error: RuntimeError) -> Self {
        let message = error.to_string();
        YawError::RuntimeError { error, message }
    }
}
