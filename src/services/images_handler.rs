// External Deps
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

// Internal Deps

const OPEN_AI_URL: &str = "https://api.openai.com/v1";
const IMAGES_ENDPOINT: &str = "/images/generations";

// MARK: OPEN_AI DEFAULTS
const DEFAULT_IMAGE_PROMPT: &str = "A Dungeons and Dragons dungeon master dressed in a cloak";
const DEFAULT_GENERATED_IMAGE_COUNT: u32 = 2;
const DEFAULT_IMAGE_RESOLUTION: u32 = 1;

// MARK: S3 DEFAULTS
const AWS_REGION: &str = "us-west-1";

pub async fn generate_images(client: Client) -> Result<String, ServiceError> {
    let test_image_count: Option<u32> = Some(1);
    let test_image_resolution_opt: Option<u32> = Some(1);

    let payload = ImagesPayload::new(
        DEFAULT_IMAGE_PROMPT,
        test_image_count,
        test_image_resolution_opt,
    );

    dbg!(&payload);

    let response = client
        .post(format!("{}{}", OPEN_AI_URL, IMAGES_ENDPOINT))
        .json(&payload)
        .send()
        .await
        .unwrap();

    dbg!(&response);

    // TODO: take blob and create presigned url with rusoto
    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body = response.json::<ImagesResponse>().await.unwrap();
            let mut result = String::new();
            response_body.data.iter().for_each(|datum| {
                result.push_str(&datum.url);
            });
            Ok(result)
        }
        _ => Err(ServiceError::new(
            "Error generating open_ai images response",
            response.status(),
        )),
    }
}

// MARK: Req Model

// TODO: Maybe put res_format at b64_json if can't upload to s3 without json
#[derive(Debug, Serialize)]
pub struct ImagesPayload {
    prompt: String,
    n: u32,
    size: String,
    // response_format: String,
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
            // response_format: "b64_json".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ImagesResponse {
    created: u64,
    data: Vec<ImageData>,
}

// TODO: Handle b64 Json data
#[derive(Debug, Deserialize)]
pub struct ImageData {
    url: String,
}

// MARK: Error handling

#[derive(Debug)]
pub struct ServiceError {
    message: String,
    status: reqwest::StatusCode,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ServiceError {
    pub fn new(message: &str, status: reqwest::StatusCode) -> Self {
        Self {
            message: message.to_string(),
            status,
        }
    }
}
