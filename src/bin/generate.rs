use async_recursion::async_recursion;

use async_std::prelude::*;

use async_std::fs;
use async_std::io::Result;
use async_std::path::Path;
use async_std::task;

use pages::app::App;

#[async_recursion(?Send)]
async fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let from = from.as_ref().to_owned();
    let to = to.as_ref().to_owned();

    let mut entries = fs::read_dir(&from).await?;
    while let Some(res) = entries.next().await {
        let entry = res?;
        let to_path = to.join(entry.file_name());
        if entry.file_type().await?.is_dir() {
            fs::create_dir_all(&to_path).await?;
            copy_dir(entry.path(), &to_path).await?;
        }
        if entry.file_type().await?.is_file() {
            fs::copy(entry.path(), &to_path).await?;
        }
    }

    return Ok(());
}

async fn prepare_static_dir<P: AsRef<Path>, Q: AsRef<Path>>(
    dist_dir: P,
    static_dir: Q,
) -> Result<()> {
    let dist_dir = dist_dir.as_ref().to_owned();
    let static_dir = static_dir.as_ref().to_owned();

    fs::remove_dir_all(&static_dir).await?;
    fs::create_dir_all(&static_dir).await?;

    let mut entries = fs::read_dir(&dist_dir).await?;
    while let Some(res) = entries.next().await {
        let entry = res?;
        if entry.path() == dist_dir.join("index.html") {
            continue;
        }

        let to = static_dir.join(entry.file_name());
        if entry.file_type().await?.is_dir() {
            fs::create_dir_all(&to).await?;
            copy_dir(entry.path(), &to).await?;
        }
        if entry.file_type().await?.is_file() {
            fs::copy(entry.path(), &to).await?;
        }
    }

    return Ok(());
}

async fn base_index_html<P: AsRef<Path>>(dist_dir: P) -> Result<(String, String)> {
    let dist_dir = dist_dir.as_ref().to_owned();

    let hydrate_html = fs::read_to_string(dist_dir.join("index.html")).await?;

    let (head, foot) = hydrate_html.split_once("<body>").unwrap();
    let mut head = head.to_owned();
    head.push_str("<body>");
    let foot = foot.to_owned();

    return Ok((head, foot));
}

async fn generate<P: AsRef<Path>, Q: AsRef<Path>>(dist_dir: P, static_dir: Q) -> Result<()> {
    let dist_dir = dist_dir.as_ref().to_owned();
    let static_dir = static_dir.as_ref().to_owned();

    prepare_static_dir(&dist_dir, &static_dir).await?;

    let (head, foot) = base_index_html(&dist_dir).await?;
    let head = head.to_owned();
    let foot = foot.to_owned();

    let renderer = yew::ServerRenderer::<App>::new();
    let rendered = renderer.render().await;

    let mut generated_html = fs::File::create(static_dir.join("index.html")).await?;

    write!(generated_html, "{}{}{}", head, rendered, foot).await?;

    return Ok(());
}

fn main() -> Result<()> {
    let dist_dir = "dist";
    let static_dir = "static";
    return task::block_on(generate(dist_dir, static_dir));
}
