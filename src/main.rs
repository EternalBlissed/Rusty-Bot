#[macro_use]
extern crate tracing;

use std::env;

use anyhow::{Context, Result};
use serenity::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;

mod config;
mod commands;
mod handler;
mod log;

// note: this value is mirrored in src/commands/help.rs
pub const EMBED_COLOR: [u8; 3] = [0x58, 0x65, 0xF2];

async fn client() -> Result<Client> {
    let token =
        env::var("DISCORD_TOKEN").context("failed to load `DISCORD_TOKEN` environment variable")?;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(token, intents)
        .event_handler(handler::Handler)
        .framework(commands::framework())
        .await
        .expect("Discord client should build successfully");

    Ok(client)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    log::logger()
        .try_init()
        .expect("logger initialization shouldn't fail");

    let mut client = client().await.context("failed to build client")?;
    client.start().await.context("client error occurred")?;

    Ok(())
}