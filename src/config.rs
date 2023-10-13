use anyhow::Context;
use lazy_static::lazy_static;
use twilight_model::id::{
    marker::{ApplicationMarker, ChannelMarker, TagMarker},
    Id,
};

fn get_env(name: &str) -> String {
    std::env::var(name)
        .context(format!(
            "Unable to find environment variable named \"{name}\"!"
        ))
        .unwrap()
}

fn get_env_id<T>(name: &str) -> Id<T> {
    Id::new(get_env(name).parse::<u64>().unwrap())
}

pub struct BotConfig {
    pub token: String,
    pub help_channel_id: Id<ChannelMarker>,
    pub unsolved_tag_id: Id<TagMarker>,
    pub solved_tag_id: Id<TagMarker>,
    pub application_id: Id<ApplicationMarker>,
}

lazy_static! {
    pub static ref CONFIG: BotConfig = BotConfig {
        token: get_env("TOKEN"),
        help_channel_id: get_env_id("HELP_CHANNEL_ID"),
        unsolved_tag_id: get_env_id("UNSOLVED_TAG_ID"),
        solved_tag_id: get_env_id("SOLVED_TAG_ID"),
        application_id: get_env_id("APPLICATION_ID"),
    };
}
