use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    #[serde(default = "default_hacker_news_url")]
    pub(crate) hacker_news_base_url: String,

    #[serde(default = "default_db_url")]
    pub(crate) db_url: String,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            .add_source(Environment::default())
            .build()?;

        cfg.try_deserialize()
    }
}

fn default_hacker_news_url() -> String {
    "https://hacker-news.firebaseio.com".into()
}

fn default_db_url() -> String {
    "./persistence/hackme.json".into()
}
