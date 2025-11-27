use std::fmt::Debug;

use crate::client::TwitterClient;
use crate::error::SdkResult;
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tweet {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TweetResponse {
    pub data: Tweet,
}

#[cfg_attr(feature = "testing", mockall::automock)]
#[async_trait]
pub trait TweetApi: Debug + Send + Sync {
    async fn get(&self, id: &str) -> SdkResult<TweetResponse>;
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
    async fn get(&self, id: &str) -> SdkResult<TweetResponse> {
        let endpoint = format!("/tweets/{}", id);
        self.client.request(Method::GET, &endpoint).await
    }
}
