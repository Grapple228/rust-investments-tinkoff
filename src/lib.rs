#![allow(unused)] // For beginning only.

// region:    --- Modules

use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

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

/// Initialization of the crate
pub fn init() -> Result<()> {
    // -- LOGGING INITIALIZATION
    tracing_subscriber::fmt()
        .without_time() // For early development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Initializing");

    // -- CONFIG INITIALIZATION
    info!("Loading config...");
    _ = config();

    Ok(())
}
