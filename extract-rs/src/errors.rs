use std::str::Utf8Error;

/// These represent recoverable errors that should be logged as part of the sync job log
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Unknown(String),

    #[error("{0}")]
    IoError(String),

    #[error("{0}")]
    ParseError(String),


    #[error("{0}")]
    JniError(#[source] jni::errors::Error),

    #[error("{0}")]
    Utf8Error(#[source] Utf8Error),
}


impl From<jni::errors::Error> for Error {
    fn from(e: jni::errors::Error) -> Self {

        // match e {
        //     jni::errors::Error::JniCall(jni_err) => {
        //         match jni_err {
        //             JniError::ThreadDetached => {}
        //             JniError::NoMemory => Error::JniError(e)
        //             JniError::Other(_) => {}
        //             _ => Error::JniError(e)
        //         }
        //     },
        //     _ => Error::JniError(e)
        // }

        Error::JniError(e)
    }
}

// #[derive(thiserror::Error, Debug)]
// pub enum TikaError {
//     #[error("Unknown error")]
//     Unknown,

//     #[error("{0}")]
//     ParseError(String),

//     #[error("{0}")]
//     JniError(String),
// }

pub type ExtractResult<T> = Result<T, Error>;