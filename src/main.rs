use anyhow::{Context, Result};
use lambda_http::{run, service_fn, Body, Error, Request, Response};

mod context;
mod db;
mod line;
mod openai;
mod secrets;

fn channel_to_context(channel: &db::Channel) -> context::Context {
    let mut ctx = context::Context::new();

    for event in &channel.events {
        let user = match &event.user {
            Some(user) => format!("User{}", user),
            None => "AI".to_string(),
        };
        ctx.speak(&user, &event.msg);
    }

    ctx
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let secrets = secrets::Secrets::new().await.get().await?;

    let bot = line::Bot::new(
        &secrets.line_channel_secret,
        &secrets.line_channel_access_token,
    )
    .context("couldn't start bot")?;

    let db = db::Db::new().await;

    let event = bot.parse(event.headers(), &event.body())?;

    let ai = openai::OpenAI::new(&secrets.openai_api_key)?;

    let mut channel = match db.get_channel(&event.channel_id).await? {
        Some(channel) => channel,
        None => db::Channel::empty(&event.channel_id),
    };

    for m in event.messages {
        channel.events.push(db::Event::user(&m.user, &m.text));

        let context = channel_to_context(&channel);

        let completion = ai.prompt(&context.as_prompt()).await?;

        bot.reply(&m.reply_token, &completion)?;

        channel.events.push(db::Event::ai(&completion));
    }

    db.set_channel(channel).await?;

    let resp = Response::builder()
        .status(200)
        .body(Body::Empty)
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
