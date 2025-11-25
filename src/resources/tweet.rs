use crate::client::TwitterClient;
use crate::error::SdkResult;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct TweetResponse {
    pub data: Tweet,
}

pub struct TweetHandler<'a> {
    client: &'a TwitterClient,
}

impl<'a> TweetHandler<'a> {
    pub fn new(client: &'a TwitterClient) -> Self {
        Self { client }
    }

    /// GET /2/tweets/:id
    pub async fn get(&self, id: &str) -> SdkResult<TweetResponse> {
        let endpoint = format!("/tweets/{}", id);
        self.client.request(Method::GET, &endpoint).await
    }
}
