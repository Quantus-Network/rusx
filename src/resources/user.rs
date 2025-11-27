use std::fmt::Debug;

use crate::client::TwitterClient;
use crate::error::SdkResult;
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserResponse {
    pub data: User,
}

/// The trait definition allows us to mock the User API interactions
#[cfg_attr(feature = "testing", mockall::automock)]
#[async_trait]
pub trait UserApi: Debug + Send + Sync {
    async fn get_me(&self) -> SdkResult<UserResponse>;
    async fn get(&self, id: &str) -> SdkResult<UserResponse>;
    async fn get_by_username(&self, username: &str) -> SdkResult<UserResponse>;
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
    async fn get_me(&self) -> SdkResult<UserResponse> {
        self.client.request(Method::GET, "/users/me").await
    }

    async fn get(&self, id: &str) -> SdkResult<UserResponse> {
        let endpoint = format!("/users/{}", id);
        self.client.request(Method::GET, &endpoint).await
    }

    async fn get_by_username(&self, username: &str) -> SdkResult<UserResponse> {
        let endpoint = format!("/users/by/username/{}", username);
        self.client.request(Method::GET, &endpoint).await
    }
}
