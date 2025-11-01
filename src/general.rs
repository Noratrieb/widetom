use std::sync::LazyLock;

use fancy_regex::Regex;
use serenity::client::Context;
use serenity::framework::standard::macros::hook;
use serenity::model::channel::{Message, ReactionType};

use crate::{Config, LastMessageInChannel};

#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    let mut data = ctx.data.write().await;
    let map = data
        .get_mut::<LastMessageInChannel>()
        .expect("LastMessageInChannel not found");
    map.insert(msg.channel_id.clone(), msg.content.clone());

    let config = data.get::<Config>().unwrap();

    static TOM_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?<=^|\D)(\d{6})(?=\D|$)").unwrap());

    let is_nsfw = msg
        .channel_id
        .to_channel(&ctx.http)
        .await
        .expect("may be nsfw lol")
        .is_nsfw();

    if let Some(m) = TOM_REGEX.find(&msg.content).unwrap() {
        if is_nsfw {
            let number = m
                .as_str()
                .parse::<u32>()
                .expect("matched regex, so it is valid");
            reply(&*format!("<https://nhentai.net/g/{}/>", number), &msg, &ctx).await;
        }
    }

    for (trigger, answer) in config.responses.iter() {
        if msg.content.to_lowercase() == *trigger {
            reply(answer, &msg, &ctx).await;
        }
    }

    for (name, id) in config.emotes.iter() {
        if msg.content.to_lowercase().contains(name) {
            if let Err(why) = msg
                .react(
                    &ctx.http,
                    ReactionType::Custom {
                        animated: false,
                        id: *id,
                        name: Some(name.to_string()),
                    },
                )
                .await
            {
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
