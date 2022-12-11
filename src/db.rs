use anyhow::Result;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub user: Option<String>,
    pub msg: String,
}

impl Event {
    pub fn user(user: &str, msg: &str) -> Self {
        Self {
            user: Some(user.into()),
            msg: msg.into(),
        }
    }

    pub fn ai(msg: &str) -> Self {
        Self {
            user: None,
            msg: msg.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    pub id: String,
    pub events: Vec<Event>,
}

impl Channel {
    pub fn empty(id: &str) -> Self {
        Self {
            id: id.into(),
            events: vec![],
        }
    }
}

pub struct Db {
    client: Client,
}

impl Db {
    const TABLE: &'static str = "channels";

    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);
        Self { client }
    }

    pub async fn set_channel(&self, channel: Channel) -> Result<()> {
        let channel = serde_dynamo::to_item(channel)?;

        self.client
            .put_item()
            .table_name(Self::TABLE)
            .set_item(Some(channel))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_channel(&self, id: &str) -> Result<Option<Channel>> {
        let output = self
            .client
            .get_item()
            .key("id", AttributeValue::S(id.to_string()))
            .table_name(Self::TABLE)
            .send()
            .await?;

        let item = output
            .item
            .map(|item| serde_dynamo::from_item(item))
            .transpose()?;

        Ok(item)
    }
}
