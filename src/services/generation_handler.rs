// External Deps
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::format::Display;
use std::io::{stdout, Write};

// Internal Deps

const OPEN_AI_URL: &str = "https://api.openai.com/v1";
const COMPLETION_ENDPOINT: &str = "/completions";

const DEFAULT_COMPLETION_MODEL: &str = "text-davinci-003";

pub async fn generate_completion(client: Client) -> Result<String, ServiveError> {}
