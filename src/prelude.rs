//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = anyhow::Result<T, Error>;
