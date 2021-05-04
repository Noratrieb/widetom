use std::collections::{HashMap, HashSet};
use std::fs;

use serenity::{async_trait, model::gateway::Ready, prelude::*};
use serenity::client::Context;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::id::{ChannelId, UserId};

use crate::commands::{ADMIN_GROUP, GENERAL_GROUP, MEME_GROUP, MY_HELP};
use crate::general::normal_message;

mod general;
mod commands;

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
    let token = fs::read_to_string("bot_token")
        .expect("Expected bot token in file 'bot_token'");

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(_) => {
            let mut owners = HashSet::new();
            owners.insert(UserId(414755070161453076)); //nils
            owners.insert(UserId(265849018662387712)); //yuki
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(false)
            .on_mention(Some(bot_id))
            .prefix("<:tom:811324632082415626> ")
            .delimiter(" ")
            .owners(owners)
        )
        .normal_message(normal_message)
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&MEME_GROUP)
        .group(&ADMIN_GROUP);

    let mut client = Client::builder(&token)
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
