use std::io;
use std::str::Utf8Error;

/// Represent errors returned by extractous
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Unknown(String),

    #[error("{0}")]
    IoError(String),

    #[error("{0}")]
    ParseError(String),

    #[error("{0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("{0}")]
    JniError(#[from] jni::errors::Error),

    #[error("{0}")]
    JniEnvCall(&'static str),
}

// Implement the conversion from our Error type to io::Error
// This allows us to use the ? when implementing std::io traits such as: Read, Write Seek etc ...
impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::IoError(msg) => {
                io::Error::new(io::ErrorKind::Other, format!("Io error: {}", msg))
            }
            Error::ParseError(msg) => {
                io::Error::new(io::ErrorKind::Other, format!("Parse error: {}", msg))
            }
            Error::Utf8Error(e) => {
                io::Error::new(io::ErrorKind::Other, format!("UTF8 error: {}", e))
            }
            Error::JniError(e) => io::Error::new(io::ErrorKind::Other, format!("JNI error: {}", e)),
            Error::JniEnvCall(msg) => {
                io::Error::new(io::ErrorKind::Other, format!("JNI env call error: {}", msg))
            }
            _ => io::Error::new(io::ErrorKind::Other, "Unknown error"),
        }
    }
}

/// Result that is a wrapper of Result<T, extractous::Error>
pub type ExtractResult<T> = Result<T, Error>;
