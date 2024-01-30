mod config;
pub use config::*;
mod env;
pub use env::*;
mod instagram;
pub use instagram::*;

fn main() {
    // Read config
    let (doc, config) = read_config("config.toml");
    println!("{:#?}", config);

    println!("Setting up...");

    // Create venv
    create_venv();

    // Install instaloader
    install_instaloader();

    download_all(&config);

    let posts = detect_all(&config);
    println!("{:#?}", posts);

    // For each new post:

    // Post image on mastodon

    // Delete image

    // Increase counter in config
}
