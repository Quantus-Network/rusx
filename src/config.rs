use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XConfig {
    pub api_base: String,
    pub oauth_url: String,
}

impl XConfig {
    pub fn load() -> Self {
        Self {
            api_base: "https://api.twitter.com/2".to_string(),
            oauth_url: "https://twitter.com/i/oauth2/authorize".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthConfig {
    pub callback_url: String,
    pub client_id: String,
    pub client_secret: String,
}
