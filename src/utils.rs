use log::info;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{Yaml, YamlLoader};

/// Load the yaml configuration for a Discord bot
pub fn load_yaml_config(filepath: &str) -> Yaml {
    let mut file = File::open(filepath).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let config = YamlLoader::load_from_str(&s).unwrap();
    config[0].clone()
}

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
