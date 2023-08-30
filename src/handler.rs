use serenity::{async_trait, model::prelude::Ready, prelude::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    #[instrument(level = "error", skip_all)]
    async fn ready(&self, _ctx: Context, bot: Ready) {
        info!(
            id = u64::from(bot.user.id),
            "Logged in as {}",
            bot.user.tag()
        );
    }
}