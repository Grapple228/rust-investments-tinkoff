// region:    --- Modules

// -- Modules
pub mod api;
pub mod channel;
pub mod extensions;

mod config;
#[cfg(feature = "datetime")]
mod datetime;
mod error;

// -- Flatten
pub use config::config;
#[cfg(feature = "datetime")]
pub use datetime::DateTime;
pub use error::{Error, Result};

// endregion: --- Modules
