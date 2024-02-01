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

pub fn upload_post(instance_domain: &str, post: Post, token: &str) -> anyhow::Result<()> {
    let client = Client::new();

    let mut parts: Vec<String> = Vec::new();
    if post.description.len() > 500 {
        let mut lines: Vec<_> = post.description.split('\n').collect();
        while !lines.is_empty() {
            let line = lines.remove(0);
            match parts.last_mut() {
                None if line.len() > 500 => {
                    let (before, after) = utf8_split(line, 500);
                    parts.push(before.to_owned());
                    lines.insert(0, after);
                },
                None => {
                    parts.push(line.to_owned())
                },
                Some(content) if content.len() + 1 + line.len() > 500 => {
                    parts.push(line.to_owned())
                },
                Some(content) => {
                    content.push('\n');
                    content.push_str(line);
                }
            }
        }
    } else {
        parts.push(post.description);
    }

    let mut previous_id = None;
    for part in parts {
        let mut form = Form::new()
            .text("status", part)
            .text("language", "fr");
        match previous_id {
            Some(id) => {
                form = form.text("in_reply_to_id", id).text("visibility", "unlisted");
            },
            None => {
                for content_path in &post.content_paths {
                    let id = upload_media(instance_domain, content_path, token, &client)?;
                    form = form.text("media_ids[]", id);
                }
                form = form.text("visibility", "public");
            }
        }

        let url = format!("https://{instance_domain}/api/v1/statuses");
        let url = Url::parse(&url)?;
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
        previous_id = Some(id.to_owned());
    }



    Ok(())
}
