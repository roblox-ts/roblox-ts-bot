use crate::{config::CONFIG, events::handle_event};
use anyhow::Result;
use dotenv::dotenv;
use futures::StreamExt;
use std::sync::Arc;
use twilight_gateway::{Intents, Shard};
use twilight_http::Client;
use twilight_model::channel::message::AllowedMentions;

mod config;
mod events;
mod jobs;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    lazy_static::initialize(&CONFIG);

    let client = Arc::new(
        Client::builder()
            .token(CONFIG.token.to_string())
            .default_allowed_mentions(AllowedMentions::default())
            .build(),
    );

    let (shard, mut events) = Shard::new(
        CONFIG.token.to_string(),
        Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    println!("Starting shard..");

    shard.start().await?;

    println!("Shard connected!");

    while let Some(event) = events.next().await {
        tokio::spawn(handle_event(client.clone(), event));
    }

    println!("Shard disconnected :(");

    Ok(())
}
