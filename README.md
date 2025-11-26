# Rusx 🦀

A modular, strongly-typed Rust SDK for the X (formerly Twitter) API v2.

**rusx** is designed to be a lightweight, ergonomic, async-first wrapper
around the Twitter API. It handles OAuth 2.0 (PKCE) authentication and
provides a clean interface for accessing resources like Tweets and
Users.

> **Note:** This project is currently in early development (**v0.1.0**).

## ✨ Features

-   **Async-First** -- built on top of `tokio` and `reqwest`.
-   **OAuth 2.0 Ready** -- native support for Authorization Code Flow
    with PKCE.
-   **Strongly Typed** -- powered by `serde` for safe parsing.
-   **Modular Architecture** -- resources (Tweets, Users, etc.)
    separated into logical modules.
-   **Error Handling** -- comprehensive error mapping for HTTP + Twitter
    API errors.

## 📦 Installation

Add `rusx` to your `Cargo.toml`:

``` toml
[dependencies]
rusx = { version = "0.1.0", path = "." }
tokio = { version = "1.0", features = ["full"] }
dotenv = "0.15"
```

## 🚀 Quick Start

### 1. Configuration & Authentication

``` rust
use rusx::{TwitterAuth, TwitterClient, PkceCodeVerifier};
use rusx::config::OauthConfig;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_config = OauthConfig {
        client_id: "YOUR_CLIENT_ID".to_string(),
        client_secret: "YOUR_CLIENT_SECRET".to_string(),
        callback_url: "http://localhost:3000/callback".to_string(),
    };

    let auth = TwitterAuth::new(auth_config)?;
    let (url, verifier) = auth.generate_auth_url();

    println!("Please open this URL in your browser:\n{}", url);
    print!("Enter the code returned in the callback URL: ");
    stdout().flush()?;

    let mut code = String::new();
    stdin().read_line(&mut code)?;

    let token_response = auth.exchange_code(code.trim().to_string(), verifier).await?;
    println!("Access Token: {}", token_response.access_token);

    let client = TwitterClient::new(token_response.access_token);

    Ok(())
}
```

### 2. Fetching a User

``` rust
let me = client.users().get_me().await?;
println!("Logged in as: @{}", me.data.username);

let musk = client.users().get_by_username("elonmusk").await?;
println!("Found user ID: {}", musk.data.id);
```

### 3. Fetching a Tweet

``` rust
let tweet = client.tweets().get("1852000000000000000").await?;
println!("Tweet text: {}", tweet.data.text);
```

## 🏗️ Architecture

### TwitterAuth

-   OAuth2 handshake
-   PKCE challenge generation
-   Authorization-code → Access-token exchange

### TwitterClient

-   Stores bearer token + HTTP client
-   Exposes resource modules: `.users()`, `.tweets()`

## 🛣️ Roadmap

-   [ ] Media Upload
-   [ ] Posting Tweets
-   [ ] Streams
-   [ ] Pagination helpers
-   [ ] Rate-limit handling

## 🤝 Contributing

1.  Fork the project\
2.  Create feature branch\
3.  Commit changes\
4.  Push branch\
5.  Open PR

## 📄 License

Apache-2.0 License.
