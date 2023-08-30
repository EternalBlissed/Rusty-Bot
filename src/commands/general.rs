use serenity::framework::standard::{macros::{command, group}, Args, CommandResult};
use serenity::model::prelude::Message;
use tokio::time::Instant;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use anyhow::Context as _;
use serenity::model::prelude::UserId;
use serenity::model::prelude::ChannelId;
use crate::config::EMBED_COLOR;


#[group]
#[commands(ping, say, userinfo, botinfo, invite, report, suggest)]
struct General;

#[command]
#[description("Shows Latency")]
#[num_args(0)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start_time = Instant::now();

    msg.reply(ctx, "Pinging...")
        .await
        .context("failed to send response message")?;

    let end_time = Instant::now();

    let latency = end_time.duration_since(start_time).as_millis();

    msg.channel_id.say(
        ctx,
        format!("Pong! Latency: {} ms", latency)
    ).await?;

    Ok(())
}

#[command]
#[description("Makes the bot say anything")]
#[usage("rsay <message>")]
async fn say(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    instrument_command!("say", msg, {
        args.trimmed().quoted();

        let reply_content = args.remains().unwrap_or("*(silence)*");

        msg.reply(ctx, reply_content)
            .await
            .context("failed to send response message")?;
        Ok(())
    })
}

#[command]
#[description("Displays information about a user")] 
#[usage("ruserinfo @barry")]
async fn userinfo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Parse the mentioned user
    let user_id = match args.single::<UserId>() {
        Ok(id) => id,
        Err(_) => {
            msg.reply(&ctx.http, "Invalid user provided.").await?;
            return Ok(());
        }
    };

    // Fetch the user's information
    let user = match user_id.to_user(&ctx.http).await {
        Ok(user) => user,
        Err(_) => {
            msg.reply(&ctx.http, "Failed to fetch user information.").await?;
            return Ok(());
        }
    };

    // Get the account creation date
    let created_at = user.created_at();

    // Create an embedded message
    let mut embed = CreateEmbed::default();
    embed.title(format!("User Info for {}", user.tag()));
    embed.color(EMBED_COLOR);
    embed.thumbnail(user.face());

    embed.field("User", format!("{}", user.name), true);
    embed.field("ID", user.id, true);
    embed.field("Bot", user.bot, true);
    embed.field("Account Created", created_at.to_string(), true);
    embed.field("Server Joined", if let Some(guild_id) = msg.guild_id {
        match guild_id.member(&ctx.http, user_id).await {
            Ok(member_option) => {
                if let member = &member_option {
                    if let Some(joined_at) = &member.joined_at {
                        joined_at.to_string()
                    } else {
                        "N/A".to_string()
                    }
                } else {
                    "N/A".to_string()
                }
            }
            Err(_) => "N/A".to_string(),
        }
    } else {
        "N/A".to_string()
    }, true);
    embed.footer(|f| {
        f.text(format!("Requested by {}", msg.author.name));
        f.icon_url(msg.author.face());
        f
    });
    

    if let Some(guild_id) = msg.guild_id {
        match guild_id.member(&ctx.http, user_id).await {
            Ok(member_option) => {
                if let member = &member_option {
                    // Additional information available only if the user is a member of the guild
                    embed.field("Nickname", &member.display_name(), true);
                    if let Some(guild) = guild_id.to_guild_cached(&ctx.cache) {
                        let roles = member.roles.iter()
                            .filter_map(|role_id| guild.roles.get(role_id).map(|role| role.name.clone()))
                            .filter(|r| !r.is_empty())
                            .collect::<Vec<_>>()
                            .join(", ");
                        embed.field("Roles", roles, true);
                    }
                }
            }
            Err(_) => {
                msg.reply(&ctx.http, "Failed to fetch member information.").await?;
                return Ok(());
            }
        }
    }

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| {
        e.clone_from(&embed);
        e
    })).await {
        eprintln!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[description("Displays detailed information about the bot. Use  rinfo")]
#[usage("rbotinfo")]
async fn botinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let owner = UserId(497682001239736329);
    let bot_name = ctx.cache.current_user().name;
    let bot_id = ctx.cache.current_user().id;
    let bot_desc = "A small project of mine written in Rust hence the name Rusty ";
    let server_count = ctx.cache.guild_count();
    let user_count = ctx.cache.user_count();
    let bot_version = "`0.15`"; 
    let website_github_link = "[Website GitHub Repository](https://example.com)"; // placeholder
    let github_link = "[GitHub Repository](https://example.com)"; // placeholder
    let recent_updates = "`Added Info commands`";

    let mut embed = CreateEmbed::default();
    embed.title("Bot Info");
    embed.colour(EMBED_COLOR);
    embed.thumbnail(ctx.cache.current_user().face());

    embed.field("Owner", format!("<@{}>", owner), true);
    embed.field("Name", bot_name, true);
    embed.field("ID", bot_id, true);
    embed.field("Description", bot_desc, false);
    embed.field("Statistics", format!("Server Count: {}\nUser Count: {}", server_count, user_count), false);
    embed.field("Version", bot_version, true);
    embed.field("Website GitHub", website_github_link, true);
    embed.field("Bot GitHub", github_link, true);
    embed.field("Recent Updates", recent_updates, false);
    embed.footer(|f| {
        f.text(format!("Requested by {}", msg.author.name));
        f.icon_url(msg.author.face());
        f
    });

    if let Err(why) = msg.channel_id.send_message(ctx, |m| m.set_embed(embed.clone())).await {
        eprintln!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[description("Get an invite link for the bot that requires Administrator permission to join a server.")]
#[usage("rinvite")]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    let invite_link = "https://discord.com/api/oauth2/authorize?client_id=1143479237882417282&permissions=8&scope=bot";
    
    let mut embed = CreateEmbed::default();
    embed.title("Bot Invite Link");
    embed.description(format!("Click [here]({}) to invite the bot to your server.", invite_link));
    embed.colour(EMBED_COLOR); // Orange color
    embed.footer(|f| {
        f.text(format!("Command Requested by: {}", msg.author.name));
        f.icon_url(msg.author.face());
        f
    });
    
    if let Err(why) = msg.channel_id.send_message(ctx, |m| m.set_embed(embed.clone())).await {
        eprintln!("Error sending message: {:?}", why);
    }
    
    Ok(())
}

#[command]
#[description("Report an issue with the bot")]
#[usage("rreport <issue>")]
#[min_args(1)]
async fn report(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let issue = args.rest(); // Get the entire string after the command
    let channel_id = ChannelId(1145565222875181136); // Change this to the correct ChannelId

    let user = &msg.author;
    let display_avatar_url = user.face();

    let mut embed = CreateEmbed::default();
    embed.color(EMBED_COLOR);
    embed.title("New Issue Report");
    embed.description(issue);
    embed.footer(|f| {
        f.text(format!("Reported by: {}", user.name));
        f.icon_url(display_avatar_url);
        f
    });

    if let Some(channel) = channel_id.to_channel(&ctx.http).await.unwrap().guild() {
        if let Err(why) = channel.send_message(&ctx.http, |m| m.set_embed(embed.clone())).await {
            eprintln!("Error sending message: {:?}", why);
        }
        msg.reply(&ctx.http, format!("Your issue has been reported in <#{}>. Thank you for your feedback!", channel_id)).await?;
    } else {
        msg.reply(&ctx.http, "Failed to find the designated channel.").await?;
    }

    Ok(())
}

#[command]
#[description("Suggest a feature for the bot")]
#[usage("rsuggest <suggestion>")]
#[min_args(1)]
async fn suggest(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let suggestion = args.rest(); // Get the entire string after the command
    let channel_id = ChannelId(1145567850896044122); // Change this to the correct ChannelId

    let user = &msg.author;
    let display_avatar_url = user.face();

    let mut embed = CreateEmbed::default();
    embed.color(EMBED_COLOR);
    embed.title("New Suggestion");
    embed.description(suggestion);
    embed.footer(|f| {
        f.text(format!("Suggested by: {}", user.name));
        f.icon_url(display_avatar_url);
        f
    });

    if let Some(channel) = channel_id.to_channel(&ctx.http).await.unwrap().guild() {
        if let Err(why) = channel.send_message(&ctx.http, |m| m.set_embed(embed.clone())).await {
            eprintln!("Error sending message: {:?}", why);
        }
        msg.reply(&ctx.http, format!("Your suggestion has been sent to <#{}>. Thank you for your feedback!", channel_id)).await?;
    } else {
        msg.reply(&ctx.http, "Failed to find the designated channel.").await?;
    }

    Ok(())
}

