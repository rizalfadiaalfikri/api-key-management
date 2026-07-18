use std::sync::LazyLock;

use anyhow::Result;
use config::{Config, Environment, File};
use serde::Deserialize;

static APP_SETTINGS: LazyLock<Settings> = LazyLock::new(
    || Settings::new().expect("Failed to load settings"),
);

pub fn jwt_secret() -> &'static str {
    &APP_SETTINGS.jwt_secret
}

pub fn jwt_expiration() -> i64 {
    APP_SETTINGS.jwt_expiration as i64
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub app_host: String,
    pub app_port: u16,
    pub rust_log: String,
    pub jwt_secret: String,
    pub jwt_expiration: u64,
}

impl Settings {
    pub fn new() -> Result<Self> {
        let settings = Config::builder()
            .add_source(File::with_name("Settings").required(false))
            .add_source(Environment::default())
            .build()?
            .try_deserialize::<Settings>()?;
        Ok(settings)
    }
}