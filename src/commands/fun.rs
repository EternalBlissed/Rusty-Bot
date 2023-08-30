use serenity::framework::standard::{macros::{command, group}, Args, CommandResult};
use serenity::model::prelude::Message;
use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use rand::Rng;
use crate::config::EMBED_COLOR;

#[group]
#[commands(eightball, guess)]
struct Fun;

#[command]
#[description("Ask the 8-Ball a question")]
#[usage("reightball <question>")]
async fn eightball(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let question = args.single::<String>().unwrap_or_default();

    if question.is_empty() {
        msg.reply(&ctx, "Invalid input. Usage: `prefix 8ball <question>`").await?;
        return Ok(());
    }

    let responses = vec![
        "Yes", "No", "It is likely", "Try again later",
        "I cannot predict that now", "Most definitely not", "Outlook good",
        "Cannot say for sure", "Chances are slim", "Maybe",
        "Better not tell you now", "My sources say no", "Without a doubt",
        "Don't count on it", "Very doubtful", "Signs point to yes",
        "Ask again later", "My reply is no", "As I see it, yes",
        "Reply hazy, try again", "Cannot predict now",
        "Concentrate and ask again", "All signs point to yes",
        "The outlook is not so good", "Definitely", "Definitely not",
        "I'm sorry, I didn't understand the question", "Ask someone else",
        "My sources say yes", "It's not looking good",
        "I'm feeling uncertain about this one", "Yes, without a doubt",
        "Don't get your hopes up", "My answer is yes, but with caution",
        "I cannot give you a clear answer at this time",
        "The stars are not aligned in your favor",
        "It's a possibility, but not a strong one", "Trust your instincts",
        "nah", "hell nah", "hell yeah", "yeah", "why not", "I ain't telling"
    ];
    let response = responses.choose(&mut rand::thread_rng()).unwrap();

    let mut embed = CreateEmbed::default();
    embed.title("8-Ball Response");
    embed.color(EMBED_COLOR); 
    embed.field("Your Question :speaking_head:", question, false);
    embed.field("8-Ball Says :8ball:", response, false);
    embed.footer(|f| {
        f.text(format!("Command Requested by: {}", msg.author.name));
        f.icon_url(msg.author.face());
        f
    });

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.clone_from(&embed);
            e
        })
    }).await?;

    Ok(())
}

#[command]
#[description("Start a customizable number guessing game")]
#[usage("rguess <min> <max> <attempts>")]
async fn guess(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let min = args.single::<i32>().unwrap_or_default();
    let max = args.single::<i32>().unwrap_or_default();
    let attempts = args.single::<i32>().unwrap_or_default();

    if min >= max || attempts <= 0 {
        msg.reply(ctx, "Invalid input. Usage: `rguess <min number> <max number> <attempts>`").await?;
        return Ok(());
    }

    let secret_number = rand::thread_rng().gen_range(min..=max);

    msg.channel_id.say(&ctx.http, format!("I'm thinking of a number between {} and {}. You have {} attempts.", min, max, attempts)).await?;

    for _ in 0..attempts {
        msg.channel_id.say(&ctx.http, format!("Enter your guess ({} attempts remaining):", attempts)).await?;

        // Wait for a message from the user
        let response = match msg.author.await_reply(&ctx).await {
            Some(response) => response,
            None => {
                msg.channel_id.say(&ctx.http, "No response received. Exiting game.").await?;
                return Ok(());
            }
        };

        // Parse the user's guess
        let guess = match response.content.parse::<i32>() {
            Ok(guess) => guess,
            Err(_) => {
                msg.channel_id.say(&ctx.http, "Invalid input. Please enter a valid number.").await?;
                continue;
            }
        };

        if guess == secret_number {
            msg.channel_id.say(&ctx.http, "Congratulations! You guessed the correct number!").await?;
            return Ok(());
        } else if guess < secret_number {
            msg.channel_id.say(&ctx.http, "Try a higher number.").await?;
        } else {
            msg.channel_id.say(&ctx.http, "Try a lower number.").await?;
        }
    }

    msg.channel_id.say(&ctx.http, format!("You've run out of attempts. The secret number was {}.", secret_number)).await?;

    Ok(())
}