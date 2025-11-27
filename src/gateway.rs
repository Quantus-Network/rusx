use crate::auth::TwitterAuth;
use crate::auth::TwitterToken;
use crate::client::TwitterClient;
use crate::config::OauthConfig;
use crate::error::SdkResult;
use crate::resources::tweet::{TweetApi, TweetHandler};
use crate::resources::user::{UserApi, UserHandler};
use async_trait::async_trait;
use oauth2::PkceCodeVerifier;
use std::fmt::Debug;
use std::sync::Arc;

/// The Main Gateway Interface
#[cfg_attr(feature = "testing", mockall::automock)]
#[async_trait]
pub trait TwitterGateway: Debug + Send + Sync {
    async fn exchange_code(
        &self,
        code: String,
        verifier: PkceCodeVerifier,
    ) -> SdkResult<TwitterToken>;

    fn generate_auth_url(&self) -> (String, PkceCodeVerifier);

    /// Access to User Resources
    fn users(&self) -> Arc<dyn UserApi>;

    /// Access to Tweet Resources
    fn tweets(&self) -> Arc<dyn TweetApi>;

    fn with_token(&self, token: String) -> SdkResult<Arc<dyn TwitterGateway>>;
}

/// The Concrete Implementation
#[derive(Clone, Debug)]
pub struct RusxGateway {
    auth: TwitterAuth,
    // We cache the handlers wrapped in Arc<dyn Trait>
    user_api: Arc<dyn UserApi>,
    tweet_api: Arc<dyn TweetApi>,
}

impl RusxGateway {
    pub fn new(oauth_config: OauthConfig, token: Option<String>) -> SdkResult<Self> {
        let auth = TwitterAuth::new(oauth_config)?;

        let bearer = token.unwrap_or_default();
        let client = TwitterClient::new(bearer);

        Ok(Self {
            auth,
            user_api: Arc::new(UserHandler::new(client.clone())),
            tweet_api: Arc::new(TweetHandler::new(client)),
        })
    }
}

#[async_trait]
impl TwitterGateway for RusxGateway {
    async fn exchange_code(
        &self,
        code: String,
        verifier: PkceCodeVerifier,
    ) -> SdkResult<TwitterToken> {
        self.auth.exchange_code(code, verifier).await
    }

    fn generate_auth_url(&self) -> (String, PkceCodeVerifier) {
        self.auth.generate_auth_url()
    }

    fn users(&self) -> Arc<dyn UserApi> {
        self.user_api.clone()
    }

    fn tweets(&self) -> Arc<dyn TweetApi> {
        self.tweet_api.clone()
    }

    fn with_token(&self, token: String) -> SdkResult<Arc<dyn TwitterGateway>> {
        let client = TwitterClient::new(token);

        let new_gateway = RusxGateway {
            auth: self.auth.clone(),
            user_api: Arc::new(UserHandler::new(client.clone())),
            tweet_api: Arc::new(TweetHandler::new(client)),
        };

        Ok(Arc::new(new_gateway))
    }
}
