# Rusty Bot
Multi Use Bot Made in Serenity

## Setting Up
- Rename .env.example to .env and fill out all fields
- Edit src/config.rs (Optional)
- Edit the prefix 'r' in main.rs (Optional)
- Run ```Cargo Run```

## Creating New Commands/Categories
### Commands
- Make sure to start every command with
```
#[command]
#[description("Your Description")]
```
And add them to the command group in your file e.g
```
#[group]
#[commands(example1, example2)]
```
### Catagorys
- Create a new file in the ```commands``` folder ending with ```.rs```
- Then Create a scaffold like this, You must have at least 1 command in the file/group because serenity is gay.
```
use serenity::framework::standard::{macros::{command, group}, Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::prelude::UserId;
use serenity::model::prelude::ChannelId;
use crate::config::EMBED_COLOR;


#[group]
#[commands(example)]
struct YourFileName;

#[command]
#[description("Shows Latency")]
#[num_args(0)]
async fn example(ctx: &Context, msg: &Message) -> CommandResult {
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
```

- Go to commands.rs and add it using ```mod YourFileName;```
- That's it, Repeat for other categories/commands
