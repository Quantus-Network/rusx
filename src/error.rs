use serde::Deserialize;
use thiserror::Error;

pub type SdkResult<T> = Result<T, SdkError>;

#[derive(Error, Debug)]
pub enum SdkError {
    #[error("Network or HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("OAuth configuration or execution error: {0}")]
    AuthConfiguration(String),

    #[error("Twitter API Error {status}: {data:?}")]
    Api {
        status: u16,
        data: TwitterApiErrorData,
    },

    #[error("Unknown error occurred: {0}")]
    Unknown(String),
}

/// Represents the standard error response body from Twitter API v2
/// Reference: https://developer.twitter.com/en/support/twitter-api/error-troubleshooting
#[derive(Debug, Deserialize)]
pub struct TwitterApiErrorData {
    pub title: String,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    // Sometimes Twitter returns specific validation errors
    #[serde(default)]
    pub errors: Option<Vec<ValidationError>>,
}

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    pub message: String,
    pub parameters: Option<serde_json::Value>,
}