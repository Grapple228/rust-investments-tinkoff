//! Crate config

use crate::error::{Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

pub fn config() -> &'static ApiConfig {
    static INSTANCE: OnceLock<ApiConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        ApiConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHOLE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ApiConfig {
    // API
    pub TINKOFF_API: String,
    pub TINKOFF_TOKEN: String,
}

impl ApiConfig {
    fn load_from_env() -> Result<ApiConfig> {
        Ok(ApiConfig {
            // API
            TINKOFF_API: get_env_parse("TINKOFF_API")
                .unwrap_or(String::from("https://invest-public-api.tinkoff.ru:443/")),
            TINKOFF_TOKEN: get_env_parse("TINKOFF_TOKEN")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;

    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}
