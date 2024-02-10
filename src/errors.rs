#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP client error")]
    SerializerError(#[from] reqwest::Error),

    #[error("Configuration error")]
    ConfigError(#[from] config::ConfigError),
}
