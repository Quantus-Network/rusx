use std::fmt::Debug;

use crate::error::SdkResult;
use crate::resources::tweet::Tweet;
use crate::resources::{
    AsQueryStr, TweetExpansion, TwitterApiResponse, UserField, join_query_param_enums_as_string,
};
use crate::{client::TwitterClient, resources::TweetField};
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchSortOrder {
    Recency,
    Relevancy,
}

impl AsQueryStr for SearchSortOrder {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Recency => "recency",
            Self::Relevancy => "relevancy",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchMeta {
    pub result_count: Option<u32>,
    pub next_token: Option<String>,
    pub newest_id: Option<String>,
    pub oldest_id: Option<String>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct SearchParams {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SearchSortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweet_fields: Option<Vec<TweetField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansions: Option<Vec<TweetExpansion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_fields: Option<Vec<UserField>>,
}

impl SearchParams {
    fn flush_query_string_buffer(
        queries: &mut Vec<String>,
        current_batch: &Vec<String>,
        keywords: Option<&str>,
    ) {
        if !current_batch.is_empty() {
            let query_string = match keywords {
                Some(k) => format!("({}) {}", current_batch.join(" OR "), k),
                None => current_batch.join(" OR "),
            };

            queries.push(query_string);
        }
    }

    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }

    pub fn build_batched_whitelist_queries(
        usernames: &Vec<String>,
        keywords: Option<&str>,
    ) -> Vec<String> {
        // Twitter's limit for the 'query' parameter
        const TWEET_SEARCH_QUERY_MAX_CHARS: usize = 512;

        let mut queries = Vec::new();

        // The query wrapper structure: "(...users...) keyword"
        // We calculate the static overhead first.
        // Overhead = "(" + ") " + keyword
        let wrapper_overhead = 1 + 2 + keywords.unwrap_or_default().len();

        let mut current_batch: Vec<String> = Vec::new();
        let mut current_len = wrapper_overhead;

        for user in usernames {
            let user_query = format!("from:{}", user);

            // Calculate length cost of adding this user
            // If it's not the first user in the batch, we need to add " OR " (4 chars)
            let separator_len = if current_batch.is_empty() { 0 } else { 4 };
            let added_len = separator_len + user_query.len();

            if current_len + added_len > TWEET_SEARCH_QUERY_MAX_CHARS {
                SearchParams::flush_query_string_buffer(&mut queries, &current_batch, keywords);

                current_batch.clear();
                current_batch.push(user_query.clone());
                current_len = wrapper_overhead + user_query.len();
            } else {
                current_batch.push(user_query);
                current_len += added_len;
            }
        }

        // Flush any remaining users in the buffer
        SearchParams::flush_query_string_buffer(&mut queries, &current_batch, keywords);

        queries
    }

    /// Helper to construct the query string manually since we are passing a string to the client wrapper
    pub fn to_query_string(&self) -> String {
        let mut params = vec![format!("query={}", urlencoding::encode(&self.query))];

        if let Some(val) = &self.start_time {
            params.push(format!("start_time={}", val));
        }
        if let Some(val) = &self.end_time {
            params.push(format!("end_time={}", val));
        }
        if let Some(val) = &self.since_id {
            params.push(format!("since_id={}", val));
        }
        if let Some(val) = &self.until_id {
            params.push(format!("until_id={}", val));
        }
        if let Some(val) = &self.max_results {
            params.push(format!("max_results={}", val));
        }
        if let Some(val) = &self.next_token {
            params.push(format!("next_token={}", val));
        }
        if let Some(val) = &self.sort_order {
            params.push(format!("sort_order={}", val.as_str()));
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
pub trait SearchApi: Debug + Send + Sync {
    /// Search for Tweets from the last 7 days
    /// Reference: https://docs.x.com/x-api/posts/search-recent-posts
    async fn recent(&self, params: SearchParams) -> SdkResult<TwitterApiResponse<Vec<Tweet>>>;

    /// Search the full archive of Tweets (requires specific access levels)
    /// Reference: https://docs.x.com/x-api/posts/search-all-posts
    async fn all(&self, params: SearchParams) -> SdkResult<TwitterApiResponse<Vec<Tweet>>>;
}

#[derive(Clone, Debug)]
pub struct SearchHandler {
    client: TwitterClient,
}

impl SearchHandler {
    pub fn new(client: TwitterClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchApi for SearchHandler {
    async fn recent(&self, params: SearchParams) -> SdkResult<TwitterApiResponse<Vec<Tweet>>> {
        let endpoint = format!("/tweets/search/recent{}", params.to_query_string());
        self.client.request(Method::GET, &endpoint).await
    }

    async fn all(&self, params: SearchParams) -> SdkResult<TwitterApiResponse<Vec<Tweet>>> {
        let endpoint = format!("/tweets/search/all{}", params.to_query_string());
        self.client.request(Method::GET, &endpoint).await
    }
}
