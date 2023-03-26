// External Deps
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::format::Display;
use std::io::{stdout, Write};

// Internal Deps

const OPEN_AI_URL: &str = "https://api.openai.com/v1";
const COMPLETIONS_ENDPOINT: &str = "/completions";

// MARK: OPEN_AI DEFAULTS
const DEFAULT_COMPLETION_MODEL: &str = "text-davinci-003";
// TODO: Look into good default for token and temperature
const DEFAULT_COMPLETION_MAX_TOKENS: u32 = 100;
const DEFAULT_COMPLETION_TEMPERATURE: f32 = 8.0;

pub async fn generate_completion(client: Client) -> Result<String, ServiveError> {
    let test_prompt = "Generate dungeons and dragons PC druid names";

    let payload = CompletionsPayload::new(&text_prompt);

    dbg!(&payload);

    let response = client
        .post(format!("{}{}", OPEN_AI_URL, COMPLETIONS_ENDPOINT))
        .json(payload)
        .send()
        .await
        .unwrap();

    dbg(&response);

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body = response.json::<CompletionsResponse>().await.unwrap();
            let mut result = String::new();
            response_body.choices.iter().for_each(|choice| {
                result.push_str(&choice.text);
            });
            Ok(result);
        }
        _ => Err(ServiceError::new(
            "Error generating open_ai completions response",
            response.status(),
        )),
    }
}

#[derive(Debug, Serialize)]
pub struct CompletionsPayload {
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

impl CompletionsPayload {
    pub fn new(
        description: &str,
        model: &Option<String>,
        tokens: Option<u32>,
        temp: Option<f32>,
    ) -> Self {
        let selected_model = match model {
            Some(m) => m.clone(),
            None => DEFAULT_COMPLETION_MODEL.to_string(),
        };

        let max_tokesn = match tokens {
            Some(t) => t,
            None => DEFAULT_COMPLETION_MAX_TOKENS,
        };

        let temperature = match temp {
            Some(t) => t,
            None => DEFAULT_COMPLETION_TEMPERATURE,
        };

        Self {
            model: selected_model,
            prompt: description.to_string(),
            max_tokens: max_tokens,
            temperature: temperature,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CompletionsResponse {
    id: String,
    object: String,
    created: u32,
    model: String,
    choices: Vec<CompletionChoices>,
}

#[derive(Debug, Deserialize)]
pub struct CompletionChoices {
    text: String,
    index: u32,
    logprobs: Option<CompletionLogProbs>,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct CompletionsLogProbs {
    token_logprobs: Vec<Vec<f32>>,
    text_offset: Vec<Vec<f32>>,
    text_logprobs: Vec<Vec<f32>>,
}

// MARK: Error Handling

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
