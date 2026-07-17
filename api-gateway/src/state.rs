use reqwest::Client;

use crate::config::{Config};

#[derive(Debug, Clone)]
pub struct AppState {

    pub config: Config,
    pub client: Client

}

impl AppState {
    pub fn new(config: Config) -> Result<Self, String> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.request_timeout_secs))
            .build()
            .map_err(|err| format!("failed to create client: {err}"))?;
        Ok(Self {
            config,
            client,
        })
    }
}