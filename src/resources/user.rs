use std::fmt::Debug;

use crate::error::SdkResult;
use crate::{client::TwitterClient, resources::TwitterApiResponse};
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub public_metrics: Option<UserPublicMetrics>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct UserPublicMetrics {
    pub followers_count: u64,
    pub following_count: u64,
    pub tweet_count: u64,
    pub listed_count: u64,
    pub like_count: Option<u64>,
    pub media_count: Option<u64>,
}

/// The trait definition allows us to mock the User API interactions
#[cfg_attr(feature = "testing", mockall::automock)]
#[async_trait]
pub trait UserApi: Debug + Send + Sync {
    async fn get_me(&self) -> SdkResult<TwitterApiResponse<User>>;
    async fn get(&self, id: &str) -> SdkResult<TwitterApiResponse<User>>;
    async fn get_by_username(&self, username: &str) -> SdkResult<TwitterApiResponse<User>>;
}

#[derive(Clone, Debug)]
pub struct UserHandler {
    client: TwitterClient,
}

impl UserHandler {
    // Accepts an owned client (cheap clone)
    pub fn new(client: TwitterClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl UserApi for UserHandler {
    async fn get_me(&self) -> SdkResult<TwitterApiResponse<User>> {
        self.client.request(Method::GET, "/users/me").await
    }

    async fn get(&self, id: &str) -> SdkResult<TwitterApiResponse<User>> {
        let endpoint = format!("/users/{}", id);
        self.client.request(Method::GET, &endpoint).await
    }

    async fn get_by_username(&self, username: &str) -> SdkResult<TwitterApiResponse<User>> {
        let endpoint = format!("/users/by/username/{}", username);
        self.client.request(Method::GET, &endpoint).await
    }
}
