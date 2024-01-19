mod config;
use config::*;
mod env;
use env::*;

fn main() {
    // Read config
    let (doc, config) = read_config("config.toml");
    println!("{:#?}", config);

    println!("Setting up...");

    // Create venv
    create_venv();

    // Install instaloader
    install_instaloader();

    // For each new post:

    // Post image on mastodon

    // Delete image

    // Increase counter in config
}
