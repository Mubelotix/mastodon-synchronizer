use std::{thread::sleep, time::Duration};
mod config;
pub use config::*;
mod env;
pub use env::*;
mod instagram;
pub use instagram::*;
mod mastodon;
pub use mastodon::*;

fn main() {
    // Read config
    let (doc, config) = read_config("config.toml");
    println!("{:#?}", config);

    println!("Setting up...");

    // Create venv
    create_venv();

    // Install instaloader
    install_instaloader();

    // Download images
    download_all(&config);

    // Detect downloaded posts
    let posts = detect_all(&config);
    println!("{:#?}", posts);

    // Upload and remove posts
    for (username, posts) in posts {
        let config = config.iter().find(|account| account.username == username).expect("Couldn't find account config");
        for post in posts {
            upload_post(&config.instance, &post, &config.token).expect("Couldn't upload post");
            delete_post(&config.instagram, post).expect("Couldn't delete post");
            sleep(Duration::from_secs(10));
        }
    }
}
