use crate::prelude::*;
use std::{fs, io::Read, path::PathBuf, sync::Arc};

use chrono::{Datelike, Local};
use reqwest::Url;
use subprocess::{Popen, PopenConfig};
use tokio::task::JoinSet;

#[derive(Debug)]
pub struct Archiver {
    accept_formats: Arc<String>,
    output_path: PathBuf,
    handles: JoinSet<()>,
}

fn get_today_string() -> String {
    let now = Local::now();
    format!(
        "{}-{:0width$}-{:0width$}",
        now.year(),
        now.month(),
        now.day(),
        width = 2
    )
}

impl Archiver {
    pub fn new(accept_formats: Vec<String>, output_path: PathBuf) -> Self {
        Self {
            accept_formats: Arc::new(accept_formats.join(",")),
            output_path,
            handles: JoinSet::new(),
        }
    }

    pub fn archive_url(&mut self, website_id: String, url: Url) {
        let accept_formats = self.accept_formats.clone();

        let output_path = self.output_path.join(website_id).join(get_today_string());

        log::info!("Archiving {}...", &url);
        self.handles.spawn(async move {
            if let Err(e) = archive_runner(url.clone(), accept_formats, output_path) {
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

macro_rules! success_return {
    ($domain:expr, $output_path:expr) => {
        log::info!("Successfully archived {} to {}", $domain, $output_path);
        fs::create_dir_all($output_path)?;
        fs::rename($domain, $output_path)?;
        return Ok(());
    };
}

fn archive_runner(url: Url, accept_formats: Arc<String>, output_path: PathBuf) -> Result<()> {
    let domain = url.domain().ok_or(Error::Archive("Invalid URL".into()))?;
    let output_path = output_path
        .as_path()
        .to_str()
        .ok_or(Error::Archive("Invalid output path".into()))?;

    let cmd = [
        "wget",
        "--accept",
        accept_formats.as_str(),
        "--mirror",
        "--tries",
        "2",
        "--no-parent",
        "--user-agent=Mozilla/5.0",
        "--convert-links",
        "--adjust-extension",
        url.as_str(),
    ];

    let mut process = Popen::create(
        &cmd,
        PopenConfig {
            stdout: subprocess::Redirection::Pipe,
            ..Default::default()
        },
    )
    .map_err(|_| Error::Archive("Failed to launch wget2".into()))?;

    let mut process_out = process
        .stdout
        .take()
        .ok_or(Error::Archive("Failed to get stdout".into()))?;

    let mut buffer = [0; 1024];
    loop {
        if process.poll().is_some() {
            success_return!(domain, output_path);
        }

        if let Err(e) = process_out.read(&mut buffer) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                success_return!(domain, output_path);
            } else {
                return Err(e.into());
            }
        }

        log::trace!("{}", String::from_utf8_lossy(&buffer));
    }
}
