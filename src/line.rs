use anyhow::{anyhow, ensure, Context, Result};
use http::header::HeaderMap;
use line_bot_sdk_rust::{
    bot::LineBot,
    events::{messages::MessageType, source::SouceType, EventType, Events},
    messages::{SendMessageType, TextMessage},
    webhook::validate_signature,
};

pub struct Event {
    pub channel_id: String,
    pub messages: Vec<Message>,
}

pub struct Message {
    pub reply_token: String,
    pub user: String,
    pub text: String,
}

pub struct Bot {
    bot: LineBot,
    channel_secret: String,
}

impl Bot {
    pub fn new(channel_secret: &str, access_token: &str) -> Result<Self> {
        let bot = LineBot::new(channel_secret, access_token);

        Ok(Self {
            channel_secret: channel_secret.into(),
            bot,
        })
    }

    pub fn parse(&self, headers: &HeaderMap, body: &[u8]) -> Result<Event> {
        let signature = headers
            .get("x-line-signature")
            .ok_or_else(|| anyhow!("signature not found"))?
            .to_str()
            .context("couldn't parse signature header as utf-8")?;

        let msg = std::str::from_utf8(body).context("couldn't parse request body as utf-8")?;

        ensure!(
            validate_signature(&self.channel_secret, signature, msg),
            "invalid signature"
        );

        let data =
            serde_json::from_str::<Events>(msg).context("couldn't parse request body as json")?;

        let msgs = data
            .events
            .iter()
            .filter_map(|e| match &e.r#type {
                EventType::MessageEvent(e) => Some(e),
                _ => None,
            })
            .filter_map(|e| match &e.message.r#type {
                MessageType::TextMessage(m) => Some((e, m)),
                _ => None,
            })
            .filter_map(|(e, m)| match &e.source.r#type {
                SouceType::User(u) => Some((e, m, u.user_id.clone())),
                _ => None,
            })
            .map(|(event, msg, user)| Message {
                reply_token: event.reply_token.clone(),
                user,
                text: msg.text.to_string(),
            })
            .collect();

        Ok(Event {
            channel_id: data.destination,
            messages: msgs,
        })
    }

    pub fn reply(&self, reply_token: &str, msg: &str) -> Result<()> {
        let message = SendMessageType::TextMessage(TextMessage {
            text: msg.to_string(),
            emojis: None,
        });

        self.bot
            .reply_message(&reply_token, vec![message])
            .context("couldn't reply")?;

        Ok(())
    }
}
