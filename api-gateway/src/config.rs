use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub auth_url: String,
    pub user_url: String,
    pub key_url: String,
    pub verify_url: String,
    pub log_url: String,
    pub request_timeout_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let port = read_env_parse("PORT", 8080)?;
        let auth_url = read_env_string("AUTH_SERVICE_URL", "http://127.0.0.1:8081");
        let user_url = read_env_string("USER_SERVICE_URL", "http://127.0.0.1:8082");
        let key_url = read_env_string("KEY_SERVICE_URL", "http://127.0.0.1:8083");
        let verify_url = read_env_string("VERIFY_SERVICE_URL", "http://127.0.0.1:8084");
        let log_url = read_env_string("LOG_SERVICE_URL", "http://127.0.0.1:8085");
        let request_timeout_secs = read_env_parse("REQUEST_TIMEOUT_SECS", 15)?;
        Ok(Self {
            port,
            auth_url,
            user_url,
            key_url,
            verify_url,
            log_url,
            request_timeout_secs,
        })
    }
}

fn read_env_string(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn read_env_parse<T>(key: &str, default: T) -> Result<T, String>
where 
    T: std::str::FromStr + Copy,
    <T as std::str::FromStr>::Err: std::fmt::Display,

{
    match env::var(key) {
        Ok(value) => value
            .parse::<T>()
            .map_err(|err| format!("env {key} tidak valid: {err}")),
        Err(_) => Ok(default),
    }
}