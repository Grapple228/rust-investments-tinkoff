//! Main Crate Error

use derive_more::derive::From;

#[cfg(feature = "datetime")]
use crate::datetime;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    // -- Modules
    #[cfg(feature = "datetime")]
    DatetimeError(datetime::Error),

    // -- Externals
    #[from]
    TransportError(tonic::transport::Error),
    #[from]
    StatusError(tonic::Status),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
