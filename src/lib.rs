// region:    --- Modules

// -- Modules
pub mod api;
pub mod channel;
#[cfg(feature = "datetime")]
mod datetime;
pub mod extensions;

mod config;
mod error;

// -- Flatten
pub use config::config;
#[cfg(feature = "datetime")]
pub use datetime::DateTime;
pub use error::{Error, Result};

// endregion: --- Modules
