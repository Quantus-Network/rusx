pub mod auth;
pub mod client;
pub mod error;
pub mod resources;

mod config;

// Re-export the Client for easy access
pub use client::TwitterClient;
pub use auth::TwitterAuth;
pub use oauth2::PkceCodeVerifier;