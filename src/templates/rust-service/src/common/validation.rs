use axum::http::HeaderMap;
use std::env;

// Error types
#[derive(Debug)]
pub enum ValidationError {
    #[allow(dead_code)]
    RapidApi(String),
    #[allow(dead_code)]
    EnvVar(String),
}

pub struct RapidApiConfig {
    pub api_key: String,
    pub proxy_secret: String,
    pub host: String,
}

impl RapidApiConfig {
    pub fn new(api_key: &str, proxy_secret: &str, host: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            proxy_secret: proxy_secret.to_string(),
            host: host.to_string(),
        }
    }

    pub fn from_env() -> Result<Self, ValidationError> {
        Ok(Self {
            api_key: env::var("RAPIDAPI_KEY")
                .map_err(|_| ValidationError::EnvVar("RAPIDAPI_KEY environment variable not set".to_string()))?,
            proxy_secret: env::var("RAPIDAPI_PROXY_SECRET")
                .map_err(|_| ValidationError::EnvVar("RAPIDAPI_PROXY_SECRET environment variable not set".to_string()))?,
            host: env::var("RAPIDAPI_HOST")
                .map_err(|_| ValidationError::EnvVar("RAPIDAPI_HOST environment variable not set".to_string()))?,
        })
    }
}

// RapidAPI validation now returns ValidationError
pub async fn validate_rapidapi_headers(
    headers: &HeaderMap,
    config: &RapidApiConfig,
) -> Result<(), ValidationError> {
    let rapidapi_key = headers
        .get("x-rapidapi-key")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ValidationError::RapidApi("Missing RapidAPI key".to_string()))?;

    if rapidapi_key != config.api_key {
        return Err(ValidationError::RapidApi("Invalid RapidAPI key".to_string()));
    }

    let rapidapi_proxy_secret = headers
        .get("x-rapidapi-proxy-secret")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ValidationError::RapidApi("Missing RapidAPI proxy secret".to_string()))?;

    if rapidapi_proxy_secret != config.proxy_secret {
        return Err(ValidationError::RapidApi("Invalid RapidAPI proxy secret".to_string()));
    }

    let rapidapi_host = headers
        .get("x-rapidapi-host")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ValidationError::RapidApi("Missing RapidAPI host".to_string()))?;

    if rapidapi_host != config.host {
        return Err(ValidationError::RapidApi("Invalid RapidAPI host".to_string()));
    }

    Ok(())
}
