use std::collections::HashMap;
use std::fs;

use fancy_regex::Regex;
use lazy_static::lazy_static;
use serenity::client::Context;
use serenity::framework::standard::{macros::hook};
use serenity::model::channel::{Message, ReactionType};
use serenity::model::id::EmojiId;
use toml::Value;

use crate::LastMessageInChannel;

pub static CONFIG_ERR: &'static str = "Invalid config file";

lazy_static! {
    pub static ref CONFIG: Value = {
        let config = fs::read_to_string("config.toml").expect("Config file not found. Add 'config.toml' to this directory");
        config.parse::<Value>().expect(CONFIG_ERR)
    };

    static ref REACTION_EMOTES: HashMap<String, EmojiId> = {
        let mut m = HashMap::new();
        let emotes = CONFIG.get("emotes").expect(CONFIG_ERR);

        for v in emotes.as_array().expect(CONFIG_ERR) {
            let name = v[0].as_str().expect(CONFIG_ERR).to_string();
            let id = EmojiId(v[1].as_integer().expect(CONFIG_ERR).clone() as u64);
            m.insert(name, id);
        }
        m
    };

    static ref RESPONSES: HashMap<String, String> = {
        let mut m = HashMap::new();

        let emotes = CONFIG.get("responses").expect(CONFIG_ERR);

        for v in emotes.as_array().expect(CONFIG_ERR) {
            let trigger = v[0].as_str().expect(CONFIG_ERR).to_string();
            let response = v[1].as_str().expect(CONFIG_ERR).to_string();
            m.insert(trigger, response);
        }
        m
    };
}

#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    let mut data = ctx.data.write().await;
    let map = data.get_mut::<LastMessageInChannel>().expect("LastMessageInChannel not found");
    map.insert(msg.channel_id.clone(), msg.content.clone());

    lazy_static! {
        static ref TOM_REGEX: Regex = Regex::new(r"(?<=^|\D)(\d{6})(?=\D|$)").unwrap();
    }

    let is_nsfw = msg.channel_id.to_channel(&ctx.http).await.expect("may be nsfw lol").is_nsfw();

    if let Some(m) = TOM_REGEX.find(&msg.content).unwrap() {
        if is_nsfw {
            let number = m.as_str().parse::<u32>().expect("matched regex, so it is valid");
            reply(&*format!("<https://nhentai.net/g/{}/>", number), &msg, &ctx).await;
        }
    }

    for (trigger, answer) in RESPONSES.iter() {
        if msg.content.to_lowercase() == *trigger {
            reply(answer, &msg, &ctx).await;
        }
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