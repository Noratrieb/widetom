use std::collections::HashSet;

use lazy_static::lazy_static;
use rand::Rng;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandGroup, CommandResult, help_commands, HelpOptions, macros::{command, group, help}};
use serenity::model::{channel::Message};
use serenity::model::id::UserId;
use serenity::utils::{content_safe, ContentSafeOptions};
use toml::Value;
use uwuifier::uwuify_str_sse;

use crate::general::{REACTION_EMOTES, CONFIG, CONFIG_ERR, reply};
use crate::LastMessageInChannel;

#[group]
#[commands(say, list)]
#[description = "General widetom commands"]
struct General;

#[group]
#[commands(uwuify, xp)]
#[description = "meme commands"]
struct Meme;

#[group]
#[commands(shutdown)]
#[owners_only]
#[description = "bot admin commands"]
struct Admin;


#[command]
#[description("lists all the commands")]
async fn list(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |msg| {
        msg.embed(|e| {
            e.title("Widetom reaction emotes");
            e.fields(REACTION_EMOTES.iter()
                .map(|em| (em.0, format!("<:{}:{}>", em.0, em.1.0), false))
            );
            e
        })
    }).await?;
    Ok(())
}

#[command]
#[description("say something")]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
        ContentSafeOptions::default()
            .clean_channel(false)
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings).await;
    msg.delete(&ctx.http).await?;
    msg.channel_id.say(&ctx.http, &content).await?;
    Ok(())
}

#[command]
#[description("uwuifies the arguments, or the last message in the channel if no args are supplied")]
async fn uwuify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(parent) = &msg.referenced_message {
        let uwu = uwuify_str_sse(&*parent.content);
        msg.channel_id.say(&ctx.http, uwu).await?;
    } else if args.is_empty() {
        let mut data = ctx.data.write().await;
        let map = data.get_mut::<LastMessageInChannel>().expect("No LastMessageInChannel in TypeMap");
        let old_message = map.get(&msg.channel_id);
        match old_message {
            Some(s) => {
                let uwu = uwuify_str_sse(s);
                msg.channel_id.say(&ctx.http, uwu).await?;
            }
            None => {
                msg.channel_id.say(&ctx.http, "Could not find last message.").await?;
            }
        }
    } else {
        let uwu = uwuify_str_sse(args.rest());
        msg.channel_id.say(&ctx.http, uwu).await?;
    }
    Ok(())
}

#[command]
#[description("end tom")]
async fn shutdown(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    reply("<:tom:811324632082415626> bye <:tom:811324632082415626>", &msg, &ctx).await;
    std::process::exit(0);
}


#[command]
#[description("display a random answer from the xp support applications")]
async fn xp(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    lazy_static! {
        static ref XP_RESPONSES: &'static Vec<Value> = CONFIG.get("xp").expect(CONFIG_ERR).as_array().expect(CONFIG_ERR);
    }
    let index = rand::thread_rng().gen_range(0..XP_RESPONSES.len());
    let random_value = XP_RESPONSES[index].as_str().expect(CONFIG_ERR);
    msg.channel_id.say(&ctx.http, random_value).await?;
    Ok(())
}

#[help]
#[individual_command_tip =
"w i d e t o m\n\n\
tom moment."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = ""]
pub async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
