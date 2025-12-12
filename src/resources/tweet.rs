use std::fmt::Debug;

use crate::error::SdkResult;
use crate::resources::{TweetExpansion, TweetField, UserField, join_query_param_enums_as_string};
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

#[derive(Debug, Serialize, Clone, Default)]
pub struct TweetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweet_fields: Option<Vec<TweetField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansions: Option<Vec<TweetExpansion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_fields: Option<Vec<UserField>>,
}

impl TweetParams {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Helper to construct the query string manually since we are passing a string to the client wrapper
    pub fn to_query_string(&self) -> String {
        let mut params = vec![];

        if let Some(ids) = &self.ids {
            params.push(format!("ids={}", ids.join(",")));
        }
        if let Some(val) = &self.tweet_fields {
            params.push(format!(
                "tweet.fields={}",
                join_query_param_enums_as_string(val)
            ));
        }
        if let Some(val) = &self.expansions {
            params.push(format!(
                "expansions={}",
                join_query_param_enums_as_string(val)
            ));
        }
        if let Some(val) = &self.user_fields {
            params.push(format!(
                "user.fields={}",
                join_query_param_enums_as_string(val)
            ));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[cfg_attr(feature = "testing", mockall::automock)]
#[async_trait]
pub trait TweetApi: Debug + Send + Sync {
    async fn get(
        &self,
        id: &str,
        params: Option<TweetParams>,
    ) -> SdkResult<TwitterApiResponse<Tweet>>;
    async fn get_many(
        &self,
        ids: Vec<String>,
        params: Option<TweetParams>,
    ) -> SdkResult<TwitterApiResponse<Tweet>>;
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
    async fn get(
        &self,
        id: &str,
        params: Option<TweetParams>,
    ) -> SdkResult<TwitterApiResponse<Tweet>> {
        let endpoint = match params {
            Some(params) => format!("/tweets/{}{}", id, params.to_query_string()),
            None => format!("/tweets/{}", id),
        };
        self.client.request(Method::GET, &endpoint).await
    }

    async fn get_many(
        &self,
        ids: Vec<String>,
        params: Option<TweetParams>,
    ) -> SdkResult<TwitterApiResponse<Tweet>> {
        let mut effective_params = TweetParams::new();

        if let Some(params) = params {
            effective_params = TweetParams { ..params };
        };

        effective_params.ids = Some(ids);

        let endpoint = format!("/tweets/{}", effective_params.to_query_string());
        self.client.request(Method::GET, &endpoint).await
    }
}
