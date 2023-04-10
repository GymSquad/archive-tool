use std::path::Path;

use chrono::{Datelike, Local};

pub fn get_collection_name() -> String {
    let now = Local::now();
    format!(
        "{}{:0width$}{:0width$}",
        now.year(),
        now.month(),
        now.day(),
        width = 2
    )
}

pub async fn init_collection(
    pywb_collections_path: &str,
    collection_name: &str,
) -> anyhow::Result<()> {
    let collection_path = Path::new(pywb_collections_path).join(collection_name);

    if !collection_path.exists() {
        tokio::fs::create_dir_all(collection_path.join("archive")).await?;
        tokio::fs::create_dir_all(collection_path.join("indexes")).await?;
        tokio::fs::create_dir_all(collection_path.join("static")).await?;
        tokio::fs::create_dir_all(collection_path.join("templates")).await?;
    }

    Ok(())
}

pub async fn add_to_collection(
    pywb_collections_path: &str,
    collection_name: &str,
    warc_file: &str,
) -> anyhow::Result<()> {
    let warc_file = format!("{}.warc.gz", warc_file);

    let archive_path = Path::new(pywb_collections_path)
        .join(collection_name)
        .join("archive")
        .join(&warc_file);

    if let Some(path) = archive_path.parent() {
        tokio::fs::create_dir_all(path).await?;
    }

    tokio::fs::rename(&warc_file, &archive_path).await?;

    Ok(())
}
