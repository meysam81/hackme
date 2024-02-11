#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP client error")]
    Serializer(#[from] reqwest::Error),

    #[error("Configuration error")]
    Config(#[from] config::ConfigError),

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Serialization error")]
    Serialization(#[from] serde_json::Error),

    #[error("UTF-8 error")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Template not found")]
    TemplateNotFound,

    #[error("Join error")]
    Join(#[from] tokio::task::JoinError),
}
