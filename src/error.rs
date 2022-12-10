//! Chargo errors

use argon2::Error as ArgonError;
use chacha20poly1305::Error as ChaChaError;
use std::io::Error as IoError;
#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<ChaChaError> for Error {
    fn from(err: ChaChaError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<ArgonError> for Error {
    fn from(err: ArgonError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}
