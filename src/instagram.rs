use crate::*;
use progress_bar::{global, init_progress_bar, enable_eta, inc_progress_bar, print_progress_bar_info, finalize_progress_bar};

fn download(instagram_username: &str) {
    run_shell_command(format!("venv/bin/instaloader --latest-stamps profiles/latest-stamps.ini --dirname-pattern=profiles/{{profile}} -- profile {instagram_username}")).expect("failed to download");
}

pub fn download_all(config: &Config) {
    init_progress_bar(config.len());
    enable_eta();
    for account in config {
        print_progress_bar_info("Downloading", &format!("@{}@instagram.com", account.instagram), progress_bar::Color::Blue, progress_bar::Style::Normal);
        download(&account.instagram);
        inc_progress_bar();
    }
    finalize_progress_bar();
}
