use std::io::{BufRead, BufReader};
use url::Url;

use subprocess::{Exec, Redirection};

pub async fn archive_url(url: &str) -> anyhow::Result<String> {
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
                print!("{}", line);
            }
            Err(e) => return Err(e.into()),
        }
    }
}
