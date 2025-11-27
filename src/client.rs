use crate::config::XConfig;
use crate::error::{SdkError, SdkResult, TwitterApiErrorData};
use crate::resources::tweet::TweetHandler;
use crate::resources::user::UserHandler;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct TwitterClient {
    http: Client,
    bearer_token: String,
    base_url: String,
}

impl TwitterClient {
    pub fn new(bearer_token: String) -> Self {
        let x_config = XConfig::load();

        Self {
            http: Client::new(),
            bearer_token,
            base_url: x_config.api_base,
        }
    }

    // Updated: Returns owned handler (no lifetimes needed)
    pub fn tweets(&self) -> TweetHandler {
        TweetHandler::new(self.clone())
    }

    // Updated: Returns owned handler
    pub fn users(&self) -> UserHandler {
        UserHandler::new(self.clone())
    }

    pub(crate) async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
    ) -> SdkResult<T> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .http
            .request(method, &url)
            .bearer_auth(&self.bearer_token)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_body = response.json::<TwitterApiErrorData>().await.map_err(|_| {
                SdkError::Unknown("Failed to parse error body from Twitter".to_string())
            })?;

            return Err(SdkError::Api {
                status: status.as_u16(),
                data: error_body,
            });
        }

        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }
}
