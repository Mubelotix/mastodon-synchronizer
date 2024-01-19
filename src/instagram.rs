use crate::*;
use progress_bar::{
    enable_eta, finalize_progress_bar, inc_progress_bar, init_progress_bar,
    print_progress_bar_info, Color, Style, set_progress_bar_action,
};

fn download(instagram_username: &str, login_username: &Option<String>) -> Result<(), ()> {
    let command = match login_username {
        Some(login_username) => format!(
            "venv/bin/instaloader --latest-stamps profiles/latest-stamps.ini --login={login_username} --sessionfile=profiles/session --dirname-pattern=profiles/{{profile}} --quiet --profile {instagram_username}"
        ),
        None => format!(
            "venv/bin/instaloader --latest-stamps profiles/latest-stamps.ini --sessionfile=profiles/session --dirname-pattern=profiles/{{profile}} --quiet --profile {instagram_username}"
        )
    };
    run_shell_command(command).map_err(|_| ())?;
    Ok(())
}

pub fn download_all(config: &Config) {
    init_progress_bar(config.len());
    enable_eta();
    set_progress_bar_action("Downloading", Color::Green, Style::Bold);
    for account in config {
        print_progress_bar_info("Downloading", &format!("{}", account.instagram), Color::Green, Style::Bold);
        if download(&account.instagram, &account.login).is_err() {
            print_progress_bar_info("Failed", &format!("download for @{}@instagram.com", account.instagram), Color::Red, Style::Bold);
        }
        inc_progress_bar();
    }
    finalize_progress_bar();
}
