// External Deps
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

// Internal Deps

const OPEN_AI_URL: &str = "https://api.openai.com/v1";
const COMPLETIONS_ENDPOINT: &str = "/images/generation";

// MARK: OPEN_AI DEFAULTS
const DEFAULT_IMAGE_PROMPT: &str = "A Dungeons and Dragons dungeon master dressed in a cloak";
const DEFAULT_GENERATED_IMAGE_COUNT: u32 = 2;
const DEFAULT_IMAGE_RESOLUTION: u32 = 1;

// MARK: Req Model

#[derive(Debug, Serialize)]
pub struct ImagesPayload {
    prompt: String,
    n: u32,
    size: String,
}

// TODO: Implement max value for matches (n is 1-10)
// TODO: Implement one two and three for size
impl ImagesPayload {
    pub fn new(
        description: &str,
        image_count: Option<u32>,
        resolution_version: Option<u32>,
    ) -> Self {
        let n = match image_count {
            Some(n) => n,
            None => DEFAULT_GENERATED_IMAGE_COUNT,
        };

        let size: String = match resolution_version {
            Some(1) => "256x256".to_string(),
            Some(2) => "512x512".to_string(),
            Some(3) => "1024x1024".to_string(),
            None => DEFAULT_IMAGE_RESOLUTION.to_string(),
            _ => DEFAULT_IMAGE_RESOLUTION.to_string(),
        };

        Self {
            prompt: description.to_string(),
            n: n,
            size: size.to_string(),
        }
    }
}
