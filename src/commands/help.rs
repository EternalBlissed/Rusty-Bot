use std::collections::HashSet;
use serenity::builder::CreateEmbed;
use serenity::utils::Color;
use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::prelude::{Message, UserId},
    prelude::*,
};

// CHANGE EMBED COLOR HEREEEEEEE
const EMBED_COLOR: Color = Color::from_rgb(255, 165, 0); // Orange color

#[help]
#[individual_command_tip("To get more information about a specific command, use `rhelp <command>`")]
#[strikethrough_commands_tip_in_dm("")]
#[strikethrough_commands_tip_in_guild("")]
#[lacking_role("strike")]
#[lacking_ownership("hide")]
#[lacking_permissions("strike")]
#[lacking_conditions("strike")]
#[wrong_channel("strike")]
#[embed_success_colour("#ffa500")]
#[embed_error_colour("#ffa500")]
#[max_levenshtein_distance(3)]
async fn help(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    instrument_command!("help", msg, {
        if args.is_empty() {
            help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await?;
        } else {
            let query = args.single::<String>().unwrap_or_default();
            let command = groups
                .iter()
                .flat_map(|group| group.options.commands.iter())
                .find(|cmd| cmd.options.names.iter().any(|name| name == &query));

            if let Some(command) = command {
                let mut embed = CreateEmbed::default();
                embed.title(format!("Command: {}", command.options.names[0]));
                embed.description(command.options.desc.unwrap_or("No description provided."));
                if let Some(usage) = &command.options.usage {
                    embed.field("Usage", usage, true); // Display the usage if available
                } else {
                    embed.field("Usage", "No information available.", true);
                }
                embed.color(EMBED_COLOR);

                msg.channel_id.send_message(ctx, |m| m.set_embed(embed)).await?;
            } else {
                msg.channel_id
                    .say(ctx, "Command not found. Use `rhelp` to list available commands.")
                    .await?;
            }
        }

        Ok(())
    })
}
