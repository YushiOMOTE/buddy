use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    model: String,
    prompt: String,
    temperature: f64,
    max_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    text: String,
    index: usize,
    finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

pub struct OpenAI {
    client: Client,
    api_key: String,
}

impl OpenAI {
    const ENDPOINT: &'static str = "https://api.openai.com/v1/completions";

    pub fn new(api_key: &str) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            api_key: api_key.into(),
        })
    }

    pub async fn prompt(&self, prompt: &str) -> Result<String> {
        let request = Request {
            model: "text-davinci-003".into(),
            prompt: prompt.into(),
            temperature: 0.1,
            max_tokens: 2048,
        };

        let response = self
            .client
            .post(Self::ENDPOINT)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .json::<Response>()
            .await?;

        Ok(response.choices[0].text.clone().trim().to_string())
    }
}
