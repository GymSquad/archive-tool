use std::{env, sync::Arc, time::Duration};

mod archive;
mod collection;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pywb_collections_path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!(
                "Usage: {} <pywb collections path>",
                std::env::args().next().unwrap()
            );
            return Ok(());
        }
    };
    let pywb_collections_path = Arc::new(pywb_collections_path);

    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Environment variable `DATABASE_URL` should be set");

    let pool = db::create_connection_pool(&database_url)
        .await
        .expect("Unable to connect to database");
    let pool = Arc::new(pool);

    let collection_name = collection::get_collection_name();
    let collection_name = Arc::new(collection_name);

    let websites = match db::get_all_urls(&pool).await {
        Ok(urls) => urls,
        Err(e) => {
            eprintln!("Unable get URLs to archive: {}", e);
            return Err(e);
        }
    };

    let mut handles = Vec::new();

    for mut website in websites.into_iter() {
        let is_valid = check_is_valid(&website.url).await;

        if is_valid != website.is_valid {
            website.is_valid = is_valid;

            if let Err(e) = db::update_website(&pool, &website).await {
                eprintln!("Unable to update website: {}", e);
                continue;
            }
        }

        if !is_valid {
            continue;
        }

        handles.push(tokio::spawn(archive_website(
            pywb_collections_path.clone(),
            collection_name.clone(),
            website.url,
        )));
    }

    for handle in handles {
        match handle.await {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => eprintln!("Unable to archive website: {}", e),
            Err(e) => eprintln!("Unable to archive website: {}", e),
        };
    }

    Ok(())
}

async fn check_is_valid(url: &str) -> bool {
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(client) => client,
        Err(_) => return false,
    };

    println!("Sending request to {}", url);
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
    println!("Archiving {}", url);
    let warc_file = archive::archive_url(&url).await?;

    tokio::fs::remove_dir_all(&warc_file).await?;

    collection::init_collection(&pywb_collections_path, &collection_name).await?;
    collection::add_to_collection(&pywb_collections_path, &collection_name, &warc_file).await?;

    Ok(())
}
