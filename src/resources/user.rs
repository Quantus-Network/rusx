use crate::client::TwitterClient;
use crate::error::SdkResult;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub data: User,
}

pub struct UserHandler<'a> {
    client: &'a TwitterClient,
}

impl<'a> UserHandler<'a> {
    pub fn new(client: &'a TwitterClient) -> Self {
        Self { client }
    }

    /// GET /2/users/me
    /// Authorization: Requires User Context (OAuth 2.0)
    pub async fn get_me(&self) -> SdkResult<UserResponse> {
        self.client.request(Method::GET, "/users/me").await
    }

    /// GET /2/users/:id
    pub async fn get(&self, id: &str) -> SdkResult<UserResponse> {
        let endpoint = format!("/users/{}", id);
        self.client.request(Method::GET, &endpoint).await
    }

    /// GET /2/users/by/username/:username
    pub async fn get_by_username(&self, username: &str) -> SdkResult<UserResponse> {
        let endpoint = format!("/users/by/username/{}", username);
        self.client.request(Method::GET, &endpoint).await
    }
}
