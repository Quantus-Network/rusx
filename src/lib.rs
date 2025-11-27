pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod gateway;
pub mod resources;

// Re-export the Client for easy access
pub use auth::TwitterAuth;
pub use client::TwitterClient;
pub use gateway::{RusxGateway, TwitterGateway};
pub use oauth2::PkceCodeVerifier;

#[cfg(feature = "testing")]
pub use gateway::MockTwitterGateway;
#[cfg(feature = "testing")]
pub use resources::tweet::MockTweetApi;
#[cfg(feature = "testing")]
pub use resources::user::MockUserApi;
