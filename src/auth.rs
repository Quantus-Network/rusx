use crate::{
    config::{OauthConfig, XConfig},
    error::{SdkError, SdkResult},
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl, basic::BasicClient,
    reqwest::async_http_client,
};
use serde::{Deserialize, Serialize};

/// A simple struct to hold the tokens returned by Twitter
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitterToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    // You can add 'scope' or 'token_type' here if needed
}

pub struct TwitterAuth {
    client: BasicClient,
}

impl TwitterAuth {
    pub fn new(oauth_config: OauthConfig) -> SdkResult<Self> {
        let x_config = XConfig::load();

        let auth_url = AuthUrl::new(x_config.oauth_url)
            .map_err(|e| SdkError::AuthConfiguration(e.to_string()))?;

        let token_url = TokenUrl::new(format!("{}/oauth2/token", x_config.api_base))
            .map_err(|e| SdkError::AuthConfiguration(e.to_string()))?;

        let client = BasicClient::new(
            ClientId::new(oauth_config.client_id),
            Some(ClientSecret::new(oauth_config.client_secret)),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new(oauth_config.callback_url)
                .map_err(|e| SdkError::AuthConfiguration(e.to_string()))?,
        );

        Ok(Self { client })
    }

    /// Generates the Authorization URL and the PKCE verifier.
    /// IMPORTANT: You must store the `PkceCodeVerifier` securely (e.g., session, redis)
    /// to use it in the callback step.
    pub fn generate_auth_url(&self) -> (String, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, _csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            // Add standard Twitter scopes here
            .add_scope(Scope::new("users.read".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (auth_url.to_string(), pkce_verifier)
    }

    /// Exchanges the authorization code for an access token.
    pub async fn exchange_code(
        &self,
        code: String,
        pkce_verifier: PkceCodeVerifier,
    ) -> SdkResult<TwitterToken> {
        let result = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|e| SdkError::AuthConfiguration(e.to_string()))?; // Mapping oauth2 errors to our SdkError

        Ok(TwitterToken {
            access_token: result.access_token().secret().to_string(),
            refresh_token: result.refresh_token().map(|t| t.secret().to_string()),
            expires_in: result.expires_in().map(|d| d.as_secs()),
        })
    }
}
