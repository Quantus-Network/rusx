use std::fmt::Debug;

use crate::error::SdkResult;
use crate::{client::TwitterClient, resources::TwitterApiResponse};
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub author_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub public_metrics: Option<TweetPublicMetrics>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TweetPublicMetrics {
    pub retweet_count: u32,
    pub reply_count: u32,
    pub like_count: u32,
    pub quote_count: u32,
    pub impression_count: u32,
    pub bookmark_count: u32,
}

#[cfg_attr(feature = "testing", mockall::automock)]
#[async_trait]
pub trait TweetApi: Debug + Send + Sync {
    async fn get(&self, id: &str) -> SdkResult<TwitterApiResponse<Tweet>>;
}

#[derive(Clone, Debug)]
pub struct TweetHandler {
    client: TwitterClient,
}

impl TweetHandler {
    pub fn new(client: TwitterClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl TweetApi for TweetHandler {
    async fn get(&self, id: &str) -> SdkResult<TwitterApiResponse<Tweet>> {
        let endpoint = format!("/tweets/{}", id);
        self.client.request(Method::GET, &endpoint).await
    }
}
