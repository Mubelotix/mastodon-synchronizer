use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use toml_edit::Document;

#[derive(Debug, Serialize, Deserialize)]
struct RawFullAccountConfig {
    instagram: Option<String>,
    token: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_delay: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum RawAccountConfig {
    Short(String),
    Full(RawFullAccountConfig),
}

#[derive(Debug, Serialize, Deserialize)]
struct RawConfigDefaults {
    refresh_delay: u32,
    instance: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawConfig {
    defaults: RawConfigDefaults,
    accounts: HashMap<String, RawAccountConfig>,
}

#[derive(Debug)]
struct AccountConfig {
    instagram: String,
    username: String,
    instance: String,
    token: String,
    refresh_delay: u32,
}

type Config = Vec<AccountConfig>;

fn read_config(path: &str) -> (Document, Config) {
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

fn main() {
    let (doc, config) = read_config("config.toml");
    println!("{:#?}", config);

    // Read config

    // Create venv

    // Install instaloader

    // For each new post:

    // Post image on mastodon

    // Delete image

    // Increase counter in config
}
