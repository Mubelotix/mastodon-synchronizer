use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::*;
use anyhow::bail;
use reqwest::{blocking::{multipart::Form, Client}, Url};

fn upload_media(instance_domain: &str, filename: &str, token: &str, client: &Client) -> anyhow::Result<String> {
    let url = format!("https://{instance_domain}/api/v2/media");
    let url = Url::parse(&url)?;
    let form = Form::new().file("file", filename)?;
    let rep = client.post(url)
        .multipart(form)
        .header("Authorization", format!("Bearer {token}"))
        .send()?;
    let status = rep.status();
    let text = rep.text()?;
    if status != 200 && status != 202 {
        bail!("Unsuccessful response: {text}")
    }
    let json = serde_json::from_str::<serde_json::Value>(&text)?;
    let Some(id) = json["id"].as_str() else {
        bail!("Unexpected type");
    };

    Ok(id.to_owned())
}

fn utf8_split(input: &str, maxsize: usize) -> (&str, &str) {
    let mut utf8_maxsize = input.len();
    if utf8_maxsize >= maxsize {
        let mut char_iter = input.char_indices();
        while utf8_maxsize >= maxsize {
            utf8_maxsize = match char_iter.next_back() {
                Some((index, _)) => index,
                _ => 0
            };
        }
        input.split_at(utf8_maxsize)
    } else {
        (input, "")
    }
}

pub fn upload_post(instance_domain: &str, post: &Post, token: &str) -> anyhow::Result<()> {
    let client = Client::new();

    // Split description into parts
    let mut parts: Vec<String> = Vec::new();
    if post.description.len() > 500 {
        let mut lines: Vec<_> = post.description.split('\n').collect();
        while !lines.is_empty() {
            let line = lines.remove(0);
            match parts.last_mut() {
                None if line.len() > 491 => {
                    let (before, after) = utf8_split(line, 491);
                    parts.push(before.to_owned());
                    lines.insert(0, after);
                },
                None => {
                    parts.push(line.to_owned())
                },
                Some(content) if content.len() + 1 + line.len() > 491 => {
                    parts.push(line.to_owned())
                },
                Some(content) => {
                    content.push('\n');
                    content.push_str(line);
                }
            }
        }
    } else {
        parts.push(post.description.to_owned());
    }

    // Split media into parts
    let mut media_parts = Vec::new();
    let image_count = post.content_paths.iter().filter(|path| path.ends_with(".jpg")).count();
    let video_count = post.content_paths.iter().filter(|path| path.ends_with(".mp4")).count();
    if video_count == 1 && image_count == 1 {
        let video = post.content_paths.iter().find(|path| path.ends_with(".mp4")).unwrap();
        media_parts.push(vec![video]);
    } else {
        if video_count > 0 {
            let videos = post.content_paths.iter().filter(|path| path.ends_with(".mp4"));
            for video in videos {
                media_parts.push(vec![video]);
            }
        }
        if image_count > 0 {
            let images = post.content_paths.iter().filter(|path| path.ends_with(".jpg")).collect::<Vec<_>>();
            for images in images.chunks(4) {
                media_parts.push(images.to_owned());
            }
        }
    }

    // Make sure parts and media_parts have the same length
    while parts.len() > media_parts.len() {
        media_parts.push(Vec::new());
    }
    while media_parts.len() > parts.len() {
        parts.push(String::new());
    }

    // Upload all media
    let mut media_ids = HashMap::new();
    for content_path in &post.content_paths {
        let id = upload_media(instance_domain, content_path, token, &client)?;
        media_ids.insert(content_path, id);
    }

    let mut previous_id = None;
    let part_len = parts.len();
    for (i, (part, media_part)) in parts.into_iter().zip(media_parts).enumerate() {
        let mut retries = 0;
        loop {
            // Add text and settings
            let mut form = Form::new();
            let part = match part_len == 1 {
                true => part.trim().to_owned(),
                false => format!("{} [{}/{part_len}]", part.trim(), i + 1)
            };
            if !part.is_empty() {
                form = form.text("status", part).text("language", "fr")
            }
            match previous_id.clone() {
                Some(id) => {
                    form = form.text("in_reply_to_id", id).text("visibility", "unlisted");
                },
                None => {
                    form = form.text("visibility", "public");
                }
            }

            // Add media
            for content_path in &media_part {
                let id = media_ids.get(content_path).expect("Couldn't find media id").clone();
                form = form.text("media_ids[]", id);
            }

            // Send request
            let url = format!("https://{instance_domain}/api/v1/statuses");
            let url = Url::parse(&url)?;
            let rep = client.post(url)
                .multipart(form)
                .header("Authorization", format!("Bearer {token}"))
                .send()?;
            let status = rep.status();
            let text = rep.text()?;
            if status != 200 && status != 202 {
                if text.contains("Réessayez dans un instant") && retries < 25 {
                    retries += 1;
                    sleep(Duration::from_secs(10));
                    continue;
                }
                bail!("Unsuccessful response: {text}")
            }
            let json = serde_json::from_str::<serde_json::Value>(&text)?;
            let Some(id) = json["id"].as_str() else {
                bail!("Unexpected type");
            };
            previous_id = Some(id.to_owned());
            break;
        }
    }

    Ok(())
}
