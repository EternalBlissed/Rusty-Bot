use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandError;
use serenity::model::prelude::UserId;
use crate::config::EMBED_COLOR;

#[group]
#[commands(kick, ban, delete)]
struct Moderation;

#[command]
#[description("Kicks a user from the server")]
#[usage("rkick <user mention> <reason>")]
#[required_permissions(KICK_MEMBERS)]   
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = match args.single::<UserId>() {
        Ok(user_id) => user_id,
        Err(err) => {
            let err_msg = format!("Invalid user provided for kick command: {:?}", err);
            error!("{}", &err_msg);
            return Err(CommandError::from(err_msg));
        }
    };
    let reason = args.rest();

    // Get the member from the user ID
    if let Ok(member) = msg.guild_id.unwrap().member(&ctx.http, user_id).await {
        if let Err(err) = member.kick(&ctx.http).await {
            let err_msg = format!("Failed to kick user: {}", err);
            error!("{}", &err_msg);
            return Err(CommandError::from(err_msg));
        }

// Successfully kicked the user
        let mut embed = CreateEmbed::default();
        embed.title("User Kicked");
        embed.description(format!("Kicked user: {}\nReason: {}", user_id, reason));
        embed.color(EMBED_COLOR); 
        embed.footer(|f| {
            f.text(format!("Requested by {}", msg.author.name));
            f.icon_url(msg.author.face());
            f
        });

        // Set other embed properties (color, footer, etc.) before sending the message
        msg.channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await?;
        info!("User kicked: {}", user_id);
    }

    Ok(())
}


#[command]
#[description("Bans a user from the server")]
#[usage("rban <user mention or ID> <reason>")]
#[required_permissions(BAN_MEMBERS)]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = match args.single::<UserId>() {
        Ok(user_id) => user_id,
        Err(err) => {
            let err_msg = format!("Invalid user provided for ban command: {:?}", err);
            error!("{}", &err_msg);
            return Err(CommandError::from(err_msg));
        }
    };
    let reason = args.rest();

    // Get the member from the user ID
    if let Ok(member) = msg.guild_id.unwrap().member(&ctx.http, user_id).await {
        if let Err(err) = member.ban_with_reason(&ctx.http, 0, reason).await {
            let err_msg = format!("Failed to ban user: {}", err);
            error!("{}", &err_msg);
            return Err(CommandError::from(err_msg));
        }

        // Successfully banned the user
        let mut embed = CreateEmbed::default();
        embed.title("User Banned");
        embed.description(format!("Banned user: {}\nReason: {}", user_id, reason));
        embed.color(EMBED_COLOR); 
        embed.footer(|f| {
            f.text(format!("Requested by {}", msg.author.name));
            f.icon_url(msg.author.face());
            f
        });

        // Set other embed properties (color, footer, etc.) before sending the message
        msg.channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await?;
        info!("User banned: {}", user_id);
    }

    Ok(())
}

#[command]
#[description("Delete a specified number of messages")]
#[usage("rdelete <count>")]
#[required_permissions(MANAGE_MESSAGES)]
async fn delete(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let count_to_delete = args.single::<u64>()?;
    
    if count_to_delete == 0 || count_to_delete > 100 {
        let _ = msg.reply(&ctx.http, "You can only delete between 1 and 100 messages at a time.").await?;
        return Ok(());
    }

    // Delete the specified number of messages
    let messages = msg
        .channel_id
        .messages(&ctx.http, |retriever| retriever.limit(count_to_delete + 1))
        .await?;
    let _ = msg.channel_id.delete_messages(&ctx.http, messages).await;

    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Successfully deleted {} messages.", count_to_delete))
        .await?;

    Ok(())
}
