use std::collections::HashMap;
use serde::Deserialize;
use toml_edit::Document;

#[derive(Debug, Deserialize)]
struct RawFullAccountConfig {
    token: String,

    #[serde(default)]
    instagram: Option<String>,

    #[serde(default)]
    instance: Option<String>,

    #[serde(default)]
    refresh_delay: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RawAccountConfig {
    Short(String),
    Full(RawFullAccountConfig),
}

#[derive(Debug, Deserialize)]
struct RawConfigDefaults {
    refresh_delay: u32,
    instance: String,
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    defaults: RawConfigDefaults,
    accounts: HashMap<String, RawAccountConfig>,
}

#[derive(Debug)]
pub struct AccountConfig {
    pub instagram: String,
    pub username: String,
    pub instance: String,
    pub token: String,
    pub refresh_delay: u32,
}

pub type Config = Vec<AccountConfig>;

pub fn read_config(path: &str) -> (Document, Config) {
    let data = std::fs::read_to_string(path).expect("Unable to read config file");
    let doc = data.parse::<Document>().expect("invalid doc");
    let raw_config: RawConfig = toml_edit::de::from_document(doc.clone()).expect("invalid config");
    let mut config = Config::new();
    for (username, raw_account_config) in raw_config.accounts {
        let raw_account_config = match raw_account_config {
            RawAccountConfig::Short(token) => RawFullAccountConfig {
                token: token,
                instagram: None,
                instance: None,
                refresh_delay: None,
            },
            RawAccountConfig::Full(full_account_config) => full_account_config,
        };
        config.push(AccountConfig {
            instagram: raw_account_config.instagram.unwrap_or_else(|| username.clone()),
            username,
            token: raw_account_config.token,
            instance: raw_account_config.instance.unwrap_or_else(|| raw_config.defaults.instance.clone()),
            refresh_delay: raw_account_config.refresh_delay.unwrap_or_else(|| raw_config.defaults.refresh_delay),
        });
    }
    (doc, config)
}
