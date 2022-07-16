use serenity::{
    framework::standard::macros::{command, group},
    framework::standard::CommandResult,
    model::channel::Message,
    prelude::*,
};

#[group]
#[prefixes("hottake", "ht")]
#[default_command(generate)]
#[commands(generate)]
pub struct Hottake;

#[command]
async fn generate(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.as_str().split(" ").collect();

    if args.len() < 2 {
        msg.reply(ctx, "Must specify an artist to make a hottake from")
            .await?;
        return Ok(());
    }

    // TODO:
    msg.reply(ctx, &args[1..].join(" ")).await?;

    Ok(())
}
