use crate::prelude::*;
use std::{path::PathBuf, sync::Arc};

use chrono::{Datelike, Local};
use reqwest::Url;
use subprocess::{Exec, Popen};
use tokio::task::JoinSet;

#[derive(Debug)]
pub struct Archiver {
    blacklist: Arc<Vec<String>>,
    output_path: PathBuf,
    handles: JoinSet<()>,
}

impl Archiver {
    pub fn new(mut blacklist: Vec<String>, mut output_path: PathBuf) -> Self {
        blacklist.iter_mut().for_each(|s| s.insert(0, '-'));

        let now = Local::now();
        output_path.push(format!(
            "{}-{:0width$}-{:0width$}",
            now.year(),
            now.month(),
            now.day(),
            width = 2
        ));

        Self {
            blacklist: Arc::new(blacklist),
            output_path,
            handles: JoinSet::new(),
        }
    }

    pub fn archive_url(&mut self, website_id: String, url: Url) {
        let blacklist = self.blacklist.clone();
        let output_path = self.output_path.join(website_id);

        self.handles.spawn(async move {
            if let Err(e) = archive_runner(url.clone(), blacklist, output_path) {
                log::error!("Failed to archive {}: {}", url, e);
            }
        });
    }

    pub async fn join_all(&mut self) {
        while self.handles.join_next().await.is_some() {}
    }

    pub async fn kill_all(&mut self) {
        self.handles.abort_all();
    }
}

fn archive_runner(url: Url, blacklist: Arc<Vec<String>>, output_path: PathBuf) -> Result<Popen> {
    let output_path = output_path.join(url.domain().unwrap());
    let output_path = output_path.as_path().to_str().unwrap();

    let mut args = vec![
        url.as_str(),
        "-O",
        output_path,
        "--mirror",
        "--max-size",
        "536870912", // 512 * 1024 * 1024 = 512 MB
        "--robots",
        "0",
        "--user-agent",
        "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/60.0",
    ];
    args.extend(blacklist.iter().map(String::as_str));

    let process = Exec::cmd("httrack")
        .args(&args)
        .popen()
        .map_err(|e| Error::Archive(e.to_string()))?;

    Ok(process)
}
