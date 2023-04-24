mod archive;
mod db;
mod error;
mod prelude;

use archive::Archiver;
use clap::Parser;
use db::Database;
use reqwest::Url;
use std::{env, path::PathBuf, time::Duration};

use crate::prelude::*;
use tokio::{fs, select, signal};

#[derive(Debug, Parser)]
struct Args {
    /// path to the file format blacklist configuration file
    #[arg(short, long)]
    blacklist: PathBuf,

    /// path to the output directory
    #[arg(short, long)]
    output: PathBuf,

    /// database URL
    #[arg(short, long)]
    database_url: Option<String>,

    /// the number of URLs to archive
    /// (useful for testing)
    /// (default: no limit)
    #[arg(short, long)]
    num_url: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    let args = Args::parse();

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let database_url = args.database_url.unwrap_or(database_url);

    let blacklist = fs::read_to_string(args.blacklist).await?;

    let mut archiver = Archiver::new(blacklist.lines().map(String::from).collect(), args.output);

    log::info!("Connecting to database...");
    let db = Database::connect(&database_url).await?;

    log::info!("Fetching URLs to archive...");
    let websites = db.get_all_urls().await?;
    let num_urls = websites.len();
    log::info!("Found {} URLs to check", num_urls);

    for website in websites.into_iter().take(args.num_url.unwrap_or(num_urls)) {
        let Ok(redirected_url) = get_redirected_url(&website.url).await else {
            log::debug!("{} is not valid", &website.url);
            if let Err(e) = db.update_validity(&website, false).await {
                log::warn!("Failed to update website in database: {}", e);
            };
            continue;
        };

        if let Err(e) = db.update_validity(&website, true).await {
            log::warn!("Failed to update website in database: {}", e);
            continue;
        };

        if website.url == redirected_url.as_str() {
            log::info!("{} is valid", &website.url);
        } else {
            log::info!("{} is valid (-> {})", &website.url, &redirected_url);
        }

        archiver.archive_url(website.id, redirected_url);
    }

    log::info!("All tasks spawned, waiting for them to finish...");

    select! {
        _ = archiver.join_all() => {
            log::info!("All tasks finished");
        }
        _ = signal::ctrl_c() => {
            log::info!("Received SIGINT, exiting...");
            archiver.kill_all().await;
        }
    }

    Ok(())
}

async fn get_redirected_url(url: &str) -> Result<Url> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client.get(url).send().await?;
    match response.status() {
        status if status.is_success() || status.is_redirection() => Ok(response.url().clone()),
        status => Err(Error::DeadUrl(status.to_string())),
    }
}
