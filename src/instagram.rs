use crate::*;
use std::collections::{HashMap, HashSet};
use progress_bar::{
    enable_eta, finalize_progress_bar, inc_progress_bar, init_progress_bar,
    print_progress_bar_info, Color, Style, set_progress_bar_action,
};
use string_tools::{get_all_after_strict, get_all_before};

fn download(instagram_username: &str, login_username: &Option<String>) -> Result<(), String> {
    let command = match login_username {
        Some(login_username) => format!(
            ".venv/bin/instaloader --latest-stamps insta/stamps.ini --login={login_username} --sessionfile=insta/session --dirname-pattern=insta/{{profile}} {instagram_username}"
        ),
        None => format!(
            ".venv/bin/instaloader --latest-stamps insta/stamps.ini --sessionfile=insta/session --dirname-pattern=insta/{{profile}} {instagram_username}"
        )
    };
    let stdout = run_shell_command(command)?;
    if let Some(error) = get_all_after_strict(&stdout, "Errors or warnings occurred:\n") {
        return Err(error.to_owned());
    }
    Ok(())
}

pub fn download_all(config: &Config) {
    init_progress_bar(config.len());
    enable_eta();
    set_progress_bar_action("Downloading", Color::Green, Style::Bold);
    for account in config {
        print_progress_bar_info("Downloading", &account.instagram, Color::Green, Style::Bold);
        if let Err(e) = download(&account.instagram, &account.login) {
            print_progress_bar_info("Failed", &format!("download for @{}@instagram.com ({e:?})", account.instagram), Color::Red, Style::Bold);
            if e.contains("Redirected to login page.") {
                print_progress_bar_info("Aborting", "due to rate limits", Color::Red, Style::Bold);
                break;
            }
        }
        inc_progress_bar();
        sleep(Duration::from_secs(10));
    }
    finalize_progress_bar();
}

#[derive(Debug)]
pub struct Post {
    id: String,
    pub description: String,
    pub content_paths: Vec<String>
}

fn detect_posts(instagram_username: &str) -> Vec<Post> {
    let Ok(readdir) = std::fs::read_dir(format!("insta/{instagram_username}")) else {
        eprintln!("Failed to read directory insta/{instagram_username}");
        return Vec::new();
    };

    // Read directory
    let mut post_ids = HashSet::new();
    let mut file_paths = HashMap::new();
    for entry in readdir {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.metadata().unwrap().is_dir() {
            continue;
        }
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.contains("_UTC") && !filename.contains("_profile_pic") {
            let post_id = get_all_before(filename, "_UTC");
            post_ids.insert(post_id.to_string());
        }
        file_paths.insert(filename.to_string(), path);
    }

    // Collect data into posts
    let mut posts = Vec::new();
    for post_id in post_ids {
        let description = std::fs::read_to_string(format!("insta/{instagram_username}/{post_id}_UTC.txt")).unwrap_or_default();
        let mut content_paths = Vec::new();
        for suffix in ["_UTC.jpg", "_UTC_1.jpg", "_UTC_2.jpg", "_UTC_3.jpg", "_UTC_4.jpg", "_UTC_5.jpg", "_UTC_6.jpg", "_UTC_7.jpg", "_UTC_8.jpg", "_UTC_9.jpg", "_UTC_10.jpg", "_UTC.mp4"].iter() {
            let filename = format!("{post_id}{suffix}");
            if let Some(path) = file_paths.get(&filename) {
                content_paths.push(path.to_str().unwrap_or("").to_string());
            }
        }
        content_paths.sort();
        posts.push(Post {
            id: post_id,
            description,
            content_paths
        });
    }

    posts.sort_by_key(|post| post.id.clone());

    posts
}

pub fn detect_all(config: &Config) -> HashMap<String, Vec<Post>> {
    let mut posts = HashMap::new();
    for account in config {
        posts.insert(account.username.clone(), detect_posts(&account.instagram));
    }
    posts
}

pub fn delete_post(instagram_username: &str, post: Post) -> anyhow::Result<()> {
    let dir_entries = std::fs::read_dir(format!("insta/{instagram_username}"))?;
    for entry in dir_entries {
        let entry = entry?;
        let path = entry.path();
        if entry.metadata()?.is_dir() {
            continue;
        }
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.starts_with(&post.id) {
            let _ = std::fs::remove_file(path);
        }
    }

    Ok(())
}
