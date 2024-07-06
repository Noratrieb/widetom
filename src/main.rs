// FIXME: Remove this allow and migrate to poise.
#![allow(deprecated)]

use std::collections::{HashMap, HashSet};
use std::fs;

use serenity::all::standard::Configuration;
use serenity::client::Context;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::id::{ChannelId, UserId};
use serenity::{async_trait, model::gateway::Ready, prelude::*};

use crate::commands::{ADMIN_GROUP, GENERAL_GROUP, MEME_GROUP, MY_HELP};
use crate::general::normal_message;

mod commands;
mod general;

pub struct LastMessageInChannel;

impl TypeMapKey for LastMessageInChannel {
    type Value = HashMap<ChannelId, String>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token_path = std::env::var("BOT_TOKEN_PATH").unwrap_or_else(|_| "bot_token".to_string());
    let token = fs::read_to_string(token_path).expect("Expected bot token in file 'bot_token'");
    let token = token.trim();

    let http = Http::new(&token);

    http.get_current_application_info()
        .await
        .expect("could not access application info");
    let bot_id = http
        .get_current_user()
        .await
        .expect("could not access the bot ID")
        .id;

    let owners = HashSet::from([
        UserId::new(414755070161453076),
        UserId::new(265849018662387712),
    ]);

    let framework = StandardFramework::new()
        .normal_message(normal_message)
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&MEME_GROUP)
        .group(&ADMIN_GROUP);
    framework.configure(
        Configuration::new()
            .with_whitespace(false)
            .on_mention(Some(bot_id))
            .prefix("<:tom:811324632082415626> ")
            .delimiter(" ")
            .owners(owners),
    );

    // We don't really need all intents, but this is a small bot so we don't care.
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<LastMessageInChannel>(HashMap::default());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
