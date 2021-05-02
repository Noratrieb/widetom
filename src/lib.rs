use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use serenity::client::Context;
use serenity::framework::standard::{macros::hook};
use serenity::model::channel::{Message, ReactionType};
use serenity::model::id::EmojiId;
use toml::Value;

lazy_static! {
    static ref REACTION_EMOTES: HashMap<String, EmojiId> = {
        let err = "Invalid config file";
        let mut m = HashMap::new();

        let config = fs::read_to_string("config.toml").expect("Config file not found. Add 'config.toml' to this directory");
        let value = config.parse::<Value>().expect(err);
        let emotes = value.get("emotes").expect(err);

        for v in emotes.as_array().expect(err) {
            let name = v[0].as_str().expect(err).to_string();
            let id = EmojiId(v[1].as_integer().expect(err).clone() as u64);
            m.insert(name, id);
        }
        m
    };
}


#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    if msg.content.to_lowercase() == "tom" {
        reply(" <:tom:811324632082415626>", &msg, &ctx).await;
    }

    for (name, id) in REACTION_EMOTES.iter() {
        if msg.content.to_lowercase().contains(name) {
            if let Err(why) = msg.react(&ctx.http, ReactionType::Custom {
                animated: false,
                id: *id,
                name: Some(name.to_string()),
            }).await {
                println!("Error reacting: {}", why);
            }
        }
    }
}

pub async fn reply(txt: &str, msg: &Message, ctx: &Context) {
    if let Err(why) = msg.channel_id.say(&ctx.http, txt).await {
        println!("Error sending message: {:?}", why);
    }
}