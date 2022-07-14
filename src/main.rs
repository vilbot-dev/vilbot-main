mod constants;
mod logger;
mod utils;

use utils::log_command;

use serenity::{
    async_trait,
    framework::standard::macros::{command, group},
    framework::standard::{CommandResult, StandardFramework},
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use constants::CONFIG;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use logger::setup_logger;

#[group]
#[commands(hottake)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} has connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    info!("Setting up logger");
    setup_logger().expect("Unable to set up logger");

    let token = &CONFIG["DISCORD"]["TOKEN"]
        .as_str()
        .expect("Couldn't find a config for DISCORD::TOKEN");
    let prefix = &CONFIG["DISCORD"]["PREFIX"]
        .as_str()
        .expect("Couldn't find a config for DISCORD::PREFIX");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&prefix))
        .group(&GENERAL_GROUP);

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

#[command]
async fn hottake(ctx: &Context, msg: &Message) -> CommandResult {
    log_command("hottake", ctx, &msg);

    let args: Vec<&str> = msg.content.as_str().split(" ").collect();

    if args.len() < 2 {
        msg.reply(ctx, "Must specify an artist to make a hottake from")
            .await?;
        return Ok(());
    }

    msg.reply(ctx, &args[1..].join(" ")).await?;

    Ok(())
}
