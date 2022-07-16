use log::info;
use serenity::model::channel::Message;
use serenity::prelude::*;

/// Log information about the invocation of a command
pub fn log_command(command_name: &str, ctx: &Context, msg: &Message) {
    info!(
        "Running command {} in guild '{}' in channel '{}' from user '{}'",
        command_name,
        match msg.guild(&ctx.cache) {
            Some(guild) => guild.name,
            None => String::new(),
        },
        msg.channel_id.to_string(),
        msg.author.name,
    );
}
