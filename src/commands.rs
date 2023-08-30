macro_rules! instrument_command {
    ($name:expr, $msg:ident, $body:block) => {{
        use serenity::model::prelude::{ChannelId, MessageId};
        use tracing::Instrument;

        async move { $body }
            .instrument(error_span!(
                $name,
                msg_id = <u64 as From<MessageId>>::from($msg.id),
                channel_id = <u64 as From<ChannelId>>::from($msg.channel_id)
            ))
            .await
    }};
}

use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::hook;
use serenity::client::Context;
use serenity::model::prelude::Message;
use serenity::framework::standard::DispatchError;

mod general;
mod help;
mod image;
mod tools;
mod fun;
mod moderation;

pub const COMMAND_PREFIX: &str = "r";

pub fn framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|cfg| cfg.prefix(COMMAND_PREFIX))
        .group(&general::GENERAL_GROUP)
        .help(&help::HELP)
        .group(&image::IMAGE_GROUP)
        .group(&tools::TOOLS_GROUP)
        .group(&fun::FUN_GROUP)
        .group(&moderation::MODERATION_GROUP)
        .on_dispatch_error(|ctx, msg, error, command_name| {
            Box::pin(dispatch_error_hook(ctx, msg, error, command_name))
        })
}       

#[hook]
async fn dispatch_error_hook(
    ctx: &Context,
    msg: &Message,
    error: DispatchError,
    command_name: &str,
) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let reply_content = format!(
                "`{COMMAND_PREFIX}{command_name}` requires {min} arguments, but only received {given}."
            );
            let reply = msg.reply_ping(ctx, reply_content).await;

            if reply.is_err() {
                error!("Unhandled dispatch error in {}: {:?}", command_name, error)
            }
        }
        DispatchError::TooManyArguments { max, given } => {
            let reply_content = format!(
                "`{COMMAND_PREFIX}{command_name}` only accepts {max} arguments, but received {given}."
            );
            let reply = msg.reply_ping(ctx, reply_content).await;

            if reply.is_err() {
                error!("Unhandled dispatch error in {}: {:?}", command_name, error)
            }
        }
        DispatchError::OnlyForDM => {
            let reply_content =
                format!("`{COMMAND_PREFIX}{command_name}` can only be used in DMs.");
            let reply = msg.reply_ping(ctx, reply_content).await;

            if reply.is_err() {
                error!("Unhandled dispatch error in {}: {:?}", command_name, error)
            }
        }
        DispatchError::OnlyForGuilds => {
            let reply_content =
                format!("`{COMMAND_PREFIX}{command_name}` can only be used in servers.");
            let reply = msg.reply_ping(ctx, reply_content).await;

            if reply.is_err() {
                error!("Unhandled dispatch error in {}: {:?}", command_name, error)
            }
        }
        DispatchError::OnlyForOwners
        | DispatchError::LackingRole
        | DispatchError::LackingPermissions(_) => {
            let reply_content =
                format!("You don't have permission to use `{COMMAND_PREFIX}{command_name}`.");
            let reply = msg.reply_ping(ctx, reply_content).await;

            if reply.is_err() {
                error!("Unhandled dispatch error in {}: {:?}", command_name, error)
            }
        }
        DispatchError::Ratelimited(_) => {
            let reply = msg
                .reply_ping(ctx, "Rate limit reached, please try again soon.")
                .await;

            if reply.is_err() {
                error!("Unhandled dispatch error in {}: {:?}", command_name, error);
            }
        }
        DispatchError::BlockedUser
        | DispatchError::BlockedGuild
        | DispatchError::BlockedChannel => {}
        _ => error!("Unhandled dispatch error in {}: {:?}", command_name, error),
    }
}
