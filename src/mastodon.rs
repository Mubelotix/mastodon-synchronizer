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

pub fn upload_post(instance_domain: &str, post: Post, token: &str) -> anyhow::Result<()> {
    let client = Client::new();

    let mut form = Form::new()
        .text("status", post.description)
        .text("language", "fr")
        .text("visibility", "public");
    for content_path in post.content_paths {
        let id = upload_media(instance_domain, &content_path, token, &client)?;
        form = form.text("media_ids[]", id);
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

    Ok(())
}
