use std::str::Utf8Error;

/// These represent recoverable errors that should be logged
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Unknown(String),

    #[error("{0}")]
    IoError(String),

    #[error("{0}")]
    ParseError(String),

    #[error("{0}")]
    Utf8Error(#[source] Utf8Error),

    #[error("{0}")]
    JniEnvCall(&'static str),

    #[error("{0}")]
    JniError(#[source] jni::errors::Error),
}


impl From<jni::errors::Error> for Error {
    fn from(e: jni::errors::Error) -> Self {
        Error::JniError(e)
    }
}

pub type ExtractResult<T> = Result<T, Error>;