use std::{env, process, sync::Arc, time::Duration};

mod archive;
mod collection;
mod db;

const PYWB_COLLECTIONS_PATH: &str = "/data";

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    let pywb_collections_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| PYWB_COLLECTIONS_PATH.to_string());
    let pywb_collections_path = Arc::new(pywb_collections_path);

    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Environment variable `DATABASE_URL` should be set");

    log::info!("Connecting to database...");
    let pool = match db::create_connection_pool(&database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            log::error!("Unable to connect to database: {}", e);
            process::exit(1);
        }
    };
    let pool = Arc::new(pool);

    let collection_name = collection::get_collection_name();
    let collection_name = Arc::new(collection_name);

    log::info!("Fetching URLs to archive...");
    let websites = match db::get_all_urls(&pool).await {
        Ok(urls) => urls,
        Err(e) => {
            log::error!("Unable get URLs to archive: {}", e);
            process::exit(1);
        }
    };
    log::info!("Found {} URLs to check", websites.len());

    let mut handles = Vec::new();

    for mut website in websites.into_iter() {
        let is_valid = check_is_valid(&website.url).await;

        if is_valid != website.is_valid {
            website.is_valid = is_valid;

            if let Err(e) = db::update_website(&pool, &website).await {
                log::warn!("Failed to update website in database: {}", e);
                continue;
            }
        }

        if !is_valid {
            continue;
        }

        log::info!("Archiving {}", &website.url);
        handles.push(tokio::spawn(archive_website(
            pywb_collections_path.clone(),
            collection_name.clone(),
            website.url,
        )));
    }

    for handle in handles {
        match handle.await {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => log::error!("Unable to archive website: {}", e),
            Err(e) => log::error!("Unable to archive website: {}", e),
        };
    }
}

async fn check_is_valid(url: &str) -> bool {
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(client) => client,
        Err(_) => return false,
    };

    match client.get(url).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

async fn archive_website(
    pywb_collections_path: Arc<String>,
    collection_name: Arc<String>,
    url: String,
) -> anyhow::Result<()> {
    let warc_file = archive::archive_url(&url).await?;

    // The name of directory get by wget is the same as the name of the WARC file, remove it.
    tokio::fs::remove_dir_all(&warc_file).await?;

    collection::init_collection(&pywb_collections_path, &collection_name).await?;

    let warc_file = format!("{}.warc.gz", warc_file);
    collection::add_to_collection(&pywb_collections_path, &collection_name, &warc_file).await?;

    Ok(())
}
