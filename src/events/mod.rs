use anyhow::Context;
use std::sync::Arc;
use twilight_gateway::Event;
use twilight_http::Client;

mod interaction_create;
mod message_create;
mod thread_create;
mod thread_update;

pub async fn handle_event(client: Arc<Client>, event: Event) {
    match event {
        Event::InteractionCreate(e) => interaction_create::handle(client, *e).await,
        Event::MessageCreate(e) => message_create::handle(client, *e).await,
        Event::ThreadCreate(e) => thread_create::handle(client, *e).await,
        Event::ThreadUpdate(e) => thread_update::handle(client, *e).await,
        _ => Ok(()),
    }
    .context("Event handler failed!")
    .unwrap();
}
