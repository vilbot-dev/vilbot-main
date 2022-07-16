mod bot_commands;
mod logger;
mod utils;

use crate::bot_commands::hottake::HOTTAKE_GROUP;
use crate::utils::log_command;

use dotenv::dotenv;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use logger::setup_logger;
use serenity::{
    async_trait,
    framework::standard::{
        macros::{group, hook},
        CommandResult, DispatchError, StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} has connected", ready.user.name);
    }
}

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    log_command(&command_name, &ctx, &msg);

    true
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => info!("Processed command '{}'", command_name),
        Err(why) => error!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    info!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    info!("Message is not a command '{}'", msg.content);
}

#[hook]
async fn delay_action(ctx: &Context, msg: &Message) {
    let _ = msg.react(ctx, '⏱').await;
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _command_name: &str) {
    if let DispatchError::Ratelimited(info) = error {
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", info.as_secs()),
                )
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    info!("Setting up logger");
    setup_logger().expect("Unable to set up logger");

    let token = env::var("DISCORD_TOKEN").expect("Couldn't find DISCORD_TOKEN");
    let prefix = env::var("DISCORD_PREFIX").expect("Couldn't find DISCORD_PREFIX");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&prefix))
        .group(&GENERAL_GROUP)
        .group(&HOTTAKE_GROUP)
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}
