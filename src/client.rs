// External Deps
use dotenv::dotenv;
use reqwest::{header, Client};
use std::env;

// Internal Deps

pub fn client_builder() -> Client {
    Client::builder()
        .default_headers(get_headers())
        .build()
        .unwrap()
}

pub fn get_headers() -> header::HeaderMap {
    let mut header_map = header::HeaderMap::new();
    let token = get_auth_token();

    header_map.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    header_map.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    header_map
}

pub fn get_auth_token() -> String {
    dotenv().ok();
    env::var("OPEN_AI_SECRET").expect("No Open AI auth token found!")
}
