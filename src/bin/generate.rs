use std::collections::HashMap;

use async_recursion::async_recursion;

use async_std::fs;
use async_std::io::Result;
use async_std::path::{Path, PathBuf};
use async_std::prelude::*;
use async_std::task;

use yew_router::Routable;

use pages::app::{ServerApp as App, ServerAppProps};
use pages::router::Route;

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

    if static_dir.exists().await {
        fs::remove_dir_all(&static_dir).await?;
    }
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

async fn render(route_path: String) -> String {
    let body = yew::ServerRenderer::<App>::with_props(move || {
        let url = yew::AttrValue::from(route_path);
        let queries = HashMap::new();
        ServerAppProps {
            url,
            queries,
        }
    })
    .render()
    .await;

    return body;
}

async fn generate<P: AsRef<Path>, Q: AsRef<Path>>(dist_dir: P, static_dir: Q) -> Result<()> {
    let dist_dir = dist_dir.as_ref().to_owned();
    let static_dir = static_dir.as_ref().to_owned();

    prepare_static_dir(&dist_dir, &static_dir).await?;

    let hydrate_html = fs::read_to_string(&dist_dir.join("index.html")).await?;

    for route in Route::routes().iter() {
        let route_path = match Route::from_path(route, &HashMap::new()) {
            Some(r) => Route::to_path(&r),
            None => match Route::not_found_route() {
                Some(r) => Route::to_path(&r),
                None => String::from("/404"),
            },
        };

        let mut file_path = PathBuf::from(&route_path.clone());
        if file_path.is_dir().await {
            file_path.push("index");
        }
        file_path.set_extension("html");
        if let Ok(path) = file_path.strip_prefix("/") {
            file_path = path.to_path_buf();
        }
        let mut generate_html = fs::File::create(static_dir.join(file_path)).await?;

        let body = render(route_path).await;

        let template_html = hydrate_html
            .replace("<!--%BODY_PLACEHOLDER%-->", &body);
        write!(generate_html, "{}", template_html).await?;
    }

    return Ok(());
}

fn main() -> Result<()> {
    let dist_dir = "dist";
    let static_dir = "static";
    return task::block_on(generate(dist_dir, static_dir));
}
