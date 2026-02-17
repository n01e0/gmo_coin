use std::{env, io, time::SystemTimeError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GmoCoinError {
    #[error("environment variable error: {0}")]
    EnvVar(#[from] env::VarError),

    #[error("http request error: {0}")]
    Ureq(#[from] ureq::Error),

    #[error("invalid header value: {0}")]
    InvalidHeaderValue(#[from] ureq::http::header::InvalidHeaderValue),

    #[error("request headers are not available")]
    MissingRequestHeaders,

    #[error("response read error: {0}")]
    ResponseRead(#[from] io::Error),

    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "async")]
    #[error("async request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("system time error: {0}")]
    SystemTime(#[from] SystemTimeError),
}

pub type Result<T> = std::result::Result<T, GmoCoinError>;
