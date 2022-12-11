use anyhow::{anyhow, Result};
use aws_sdk_secretsmanager::Client;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Keys {
    pub line_channel_access_token: String,
    pub line_channel_secret: String,
    pub openai_api_key: String,
}

pub struct Secrets {
    client: Client,
}

impl Secrets {
    const SECRETS_NAME: &'static str = "buddy";

    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);
        Self { client }
    }

    pub async fn get(&self) -> Result<Keys> {
        let resp = self
            .client
            .get_secret_value()
            .secret_id(Self::SECRETS_NAME)
            .send()
            .await?;

        let s = resp
            .secret_string
            .ok_or_else(|| anyhow!("secrets string not found"))?;
        let keys = serde_json::from_str(&s)?;

        Ok(keys)
    }
}
