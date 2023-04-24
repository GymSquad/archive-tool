#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(#[from] std::env::VarError),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error("Failed to send request: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Dead URL: {0}")]
    DeadUrl(String),

    #[error("Failed to archive URL: {0}")]
    Archive(String),
}
