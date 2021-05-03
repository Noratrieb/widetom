use std::collections::HashSet;
use std::fs;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandGroup, CommandResult, help_commands, HelpOptions, macros::{command, group, help}};
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::id::UserId;
use serenity::utils::{content_safe, ContentSafeOptions};
use uwuifier::uwuify_str_sse;

use widertom::{normal_message, reply};

#[group]
#[commands(say)]
#[description = "General widetom commands"]
struct General;

#[group]
#[commands(uwuify)]
#[description = "meme commands"]
struct Meme;

#[group]
#[commands(shutdown)]
#[owners_only]
#[description = "bot admin commands"]
struct Admin;

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

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
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
    msg.channel_id.say(&ctx.http, &content).await?;
    Ok(())
}

#[command]
async fn uwuify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let uwu = uwuify_str_sse(args.rest());
    msg.channel_id.say(&ctx.http, uwu).await?;
    Ok(())
}

#[command]
async fn shutdown(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    reply("<:tom:811324632082415626> bye <:tom:811324632082415626>", &msg, &ctx).await;
    std::process::exit(0);
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
