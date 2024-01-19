use crate::*;
use progress_bar::{
    enable_eta, finalize_progress_bar, inc_progress_bar, init_progress_bar,
    print_progress_bar_info, Color, Style, set_progress_bar_action,
};

fn download(instagram_username: &str) -> Result<(), ()> {
    run_shell_command(
        format!(
            "venv/bin/instaloader --latest-stamps profiles/latest-stamps.ini --dirname-pattern=profiles/{{profile}} -- profile {instagram_username}"
        ))
        .map_err(|_| ())?;
    Ok(())
}

pub fn download_all(config: &Config) {
    init_progress_bar(config.len());
    enable_eta();
    set_progress_bar_action("Downloading", Color::Green, Style::Bold);
    for account in config {
        print_progress_bar_info("Downloading", &format!("{}", account.instagram), Color::Green, Style::Bold);
        if download(&account.instagram).is_err() {
            print_progress_bar_info("Failed", &format!("download for @{}@instagram.com", account.instagram), Color::Red, Style::Bold);
        }
        inc_progress_bar();
    }
    finalize_progress_bar();
}
