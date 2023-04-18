use lazy_static::lazy_static;
use regex::Regex;
use std::io::{BufRead, BufReader};
use url::Url;

use subprocess::{Exec, Redirection};

pub async fn archive_url(url: &str) -> anyhow::Result<String> {
    lazy_static! {
        static ref URL_REGEX: Regex = Regex::new("URL:[^ ]*").unwrap();
    }

    let warc_file = Url::parse(url)?.host_str().unwrap_or("default").to_string();

    let out = Exec::cmd("wget")
        .args(&[
            "--reject-regex",
            "(.*)\\?(.*)",
            "--mirror",
            "--warc-file",
            &warc_file,
            "--user-agent",
            "Mozilla",
            "--no-verbose",
            url,
        ])
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .stream_stdout()?;

    let mut line = String::new();
    let mut reader = BufReader::new(out);
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => return Ok(warc_file),
            Ok(_) => {
                for cap in URL_REGEX.captures_iter(&line) {
                    log::info!("Fetched {}", &cap[0]);
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
}
