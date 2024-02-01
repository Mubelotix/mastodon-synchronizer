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

    download_all(&config);

    let posts = detect_all(&config);
    println!("{:#?}", posts);

    for (username, posts) in posts {
        let config = config.iter().find(|account| account.username == username).expect("Couldn't find account config");
        for post in posts {
            upload_post(&config.instance, post, &config.token).expect("Couldn't upload post");
            break;
        }
        break;
    }

    // For each new post:

    // Post image on mastodon

    // Delete image

    // Increase counter in config
}
