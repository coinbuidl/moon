#![allow(dead_code)]

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OcOptimError {
    #[error("openclaw binary unavailable: {0}")]
    MissingOpenClawBinary(String),
    #[error("config file invalid or unreadable: {0}")]
    InvalidConfig(String),
    #[error("deterministic failure after retries: {0}")]
    DeterministicFailure(String),
}
