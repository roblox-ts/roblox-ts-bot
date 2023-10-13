use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::{
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, ReactionType,
    },
    gateway::payload::incoming::MessageCreate,
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::util::DELETE_BUTTON_ID;

lazy_static! {
    static ref PLAYGROUND_REGEX: Regex =
        Regex::new(r"^\s*https://roblox-ts\.com/playground/#code/[A-Za-z0-9\-\+]+\s*$").unwrap();
}

pub async fn handle(client: Arc<Client>, event: MessageCreate) -> Result<()> {
    let content = &event.content;

    if !PLAYGROUND_REGEX.is_match(content) {
        return Ok(());
    }

    let embeds = [EmbedBuilder::new()
        .title("Playground link")
        .url(content.trim())
        .description(format!("Posted by <@{}>", event.author.id))
        .color(0xE2_24_1A)
        .build()];

    let components = [Component::ActionRow(ActionRow {
        components: Vec::from([Component::Button(Button {
            custom_id: Some(String::from(DELETE_BUTTON_ID)),
            disabled: false,
            emoji: Some(ReactionType::Unicode {
                name: (String::from("🗑")),
            }),
            label: Some(String::from("Delete")),
            style: ButtonStyle::Danger,
            url: None,
        })]),
    })];

    let mut message = client
        .create_message(event.channel_id)
        .embeds(&embeds)
        .context("Invalid embeds")?
        .components(&components)
        .context("Invalid components")?;

    if let Some(referenced_message) = &event.referenced_message {
        message = message.reply(referenced_message.id);
    }

    message.await.context("Failed to send playground embed")?;
    // if the bot message fails, exit (with ?) before deleting the original message

    client
        .delete_message(event.channel_id, event.id)
        .await
        .context("Failed to delete original playground link")?;

    println!(
        "Created embedded playground link for {}",
        &event.author.name
    );

    Ok(())
}
