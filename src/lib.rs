// region:    --- Modules

// -- Modules
pub mod api;
pub mod channel;
pub mod extensions;

mod config;
mod error;

// -- Flatten
pub use config::config;
pub use error::{Error, Result};

// endregion: --- Modules
