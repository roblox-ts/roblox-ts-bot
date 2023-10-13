use anyhow::{Context, Result};
use std::sync::Arc;
use twilight_http::Client;
use twilight_mention::ParseMention;
use twilight_model::{
    application::interaction::{
        message_component::MessageComponentInteractionData, Interaction, InteractionData,
    },
    channel::message::{Embed, MessageFlags},
    gateway::payload::incoming::InteractionCreate,
    guild::PartialMember,
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::Id,
    user::User,
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::config::CONFIG;
use crate::util::DELETE_BUTTON_ID;

pub async fn handle(client: Arc<Client>, event: InteractionCreate) -> Result<()> {
    let InteractionCreate(Interaction {
        id,
        token,
        message: Some(message),
        data:
            Some(InteractionData::MessageComponent(MessageComponentInteractionData {
                custom_id: button_id,
                ..
            })),
        member:
            Some(PartialMember {
                user: Some(User { id: user_id, .. }),
                ..
            }),
        ..
    }) = event
    else {
        return Ok(());
    };

    if button_id != DELETE_BUTTON_ID {
        return Ok(());
    }

    let Some(Embed {
        description: Some(desc),
        ..
    }) = message.embeds.first()
    else {
        return Ok(());
    };
    let Some(playground_author) = Id::iter(desc).next() else {
        return Ok(());
    };

    if user_id == playground_author.0 {
        client
            .delete_message(message.channel_id, message.id)
            .await
            .context("Failed to delete playground embed")?;
    } else {
        let interaction_client = client.interaction(CONFIG.application_id);
        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(format!("Only <@{}> can do that!", playground_author.0))
                    .flags(MessageFlags::EPHEMERAL)
                    .build(),
            ),
        };
        interaction_client
            .create_response(id, &token, &response)
            .await
            .context("Failed to reject playground embed delete (member requesting =/= author)")?;
    }

    Ok(())
}
