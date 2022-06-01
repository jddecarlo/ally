pub mod commands;
pub(crate) mod utilities;

use anyhow::Result;
use thiserror::Error;

pub type AllyResult<T> = Result<T, AllyError>;

#[derive(Error, Debug)]
pub enum AllyError {
    #[error("Unknown error.")]
    Unknown,

    #[error("An IO error occurred.")]
    IOError(#[from] std::io::Error),

    #[error("An error occurred while parsing a JSON string.")]
    JsonError(#[from] serde_json::Error),

    #[error("Failed to fetch.")]
    GitFailedToFetch,

    #[error("Failed to get commits.")]
    GitFailedToGetCommits,
}
