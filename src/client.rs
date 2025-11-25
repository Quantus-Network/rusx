use crate::config::XConfig;
use crate::error::{SdkError, SdkResult, TwitterApiErrorData};
use crate::resources::tweet::TweetHandler;
use crate::resources::user::UserHandler;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct TwitterClient {
    http: Client,
    bearer_token: String,
    base_url: String,
}

impl TwitterClient {
    pub fn new(bearer_token: String) -> SdkResult<Self> {
        let x_config = XConfig::load();

        Ok(Self {
            http: Client::new(),
            bearer_token,
            base_url: x_config.api_base,
        })
    }

    pub fn tweets(&'_ self) -> TweetHandler<'_> {
        TweetHandler::new(self)
    }

    pub fn users(&'_ self) -> UserHandler<'_> {
        UserHandler::new(self)
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
            // Try to parse the error body provided by Twitter
            let error_body = response.json::<TwitterApiErrorData>().await.map_err(|_| {
                // If we can't parse the JSON, return a generic error
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
