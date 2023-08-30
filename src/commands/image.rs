use reqwest::Client as HttpClient;
use serenity::framework::standard::{macros::{command, group}, CommandResult};
use serenity::model::prelude::Message;
use rand::{seq::SliceRandom, rngs::StdRng, SeedableRng};
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use crate::config::EMBED_COLOR;

#[group]
#[commands(aibeach, shiba)]
struct Image;

// Image commands
#[command]
#[description("Shows a random shiba")]
#[num_args(0)]
async fn shiba(ctx: &Context, msg: &Message) -> CommandResult {
    // Construct the URL for the shibe API
    let count = 1; // Number of images to retrieve
    let urls = true; // Whether to include image URLs
    let https_urls = true; // Whether to use HTTPS URLs
    let api_url = format!("http://shibe.online/api/shibes?count={}&urls={}&httpsUrls={}", count, urls, https_urls);

    // Make a GET request to the shibe API
    let http_client = HttpClient::new();
    let response = http_client.get(&api_url).send().await?;

    // Check if the response was successful
    if response.status().is_success() {
        let image_urls: Vec<String> = response.json().await?;

        let mut rng = StdRng::from_entropy();
        if let Some(random_url) = image_urls.choose(&mut rng) {
            // Create an embedded message
            let mut embed = CreateEmbed::default(); // Create a mutable embed
            embed.title("Here's a shiba!");
            embed.image(random_url);
            embed.color(EMBED_COLOR); 
            embed.footer(|f| {
                f.text(format!("Requested by {}", msg.author.name));
                f.icon_url(msg.author.face());
                f
            });

            // Send the embedded message
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.clone_from(&embed); // Clone the embed into the closure
                    e
                })
            }).await?;
        } else {
            msg.reply(&ctx.http, "No images found.").await?;
        }
    } else {
        msg.reply(&ctx.http, "Failed to retrieve images.").await?;
    }

    Ok(())
}

#[command]
#[description("Shows a random beach image")]
#[num_args(0)]
async fn aibeach(ctx: &Context, msg: &Message) -> CommandResult {
    let image_urls = vec![
        "https://i.ibb.co/5sX6N82/DALL-E-2022-09-06-12-28-45-Beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/0jxFd0Y/DALL-E-2022-09-06-12-44-01-man-standing-on-vaperwave-styled-beach-realistic.png",
        "https://i.ibb.co/wQxqBW1/DALL-E-2022-09-06-12-46-05-Beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/YP6Vrrg/DALL-E-2022-09-06-19-52-46-Beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/7WP94MY/DALL-E-2022-09-06-19-52-53-Beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/qkJGhYN/DALL-E-2022-09-06-19-52-56-Beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/Nt4SdPP/DALL-E-2022-09-06-19-53-09-vaperwave-style-beach-realistic.png",
        "https://i.ibb.co/16Ck6wG/DALL-E-2022-09-06-19-53-13-vaperwave-style-beach-realistic.png",
        "https://i.ibb.co/VphX7ZV/DALL-E-2022-09-06-19-53-24-Beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/bFQ58RC/DALL-E-2022-09-06-19-53-26-Beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/GH0RQjT/DALL-E-2022-10-10-11-35-08-Vaperwave-style-beach-realistic.png",
        "https://i.ibb.co/2dJX5tH/DALL-E-2022-10-10-11-51-42-beach-with-pyramids-in-the-ocean.png",
        "https://i.ibb.co/1r056ZX/DALL-E-2022-10-10-11-54-50-extend.png",
        "https://i.ibb.co/xmZ09SL/DALL-E-2022-10-10-11-56-43-beach-in-vaperwave-style-digital-art.png",
        "https://i.ibb.co/VHd9yTt/DALL-E-2022-10-10-11-57-16-beach-in-tokyo-digital-art.png",
        "https://i.ibb.co/WgYTYs9/DALL-E-2023-03-06-12-34-39-Synthwave-Vaperwave-Cyberpunk-styled-beach-pink-sand-aqua-sky-vaperwave-s.png",
        "https://i.ibb.co/TgxQzN2/DALL-E-2023-03-06-12-34-44-Synthwave-Vaperwave-Cyberpunk-styled-beach-pink-sand-aqua-sky-vaperwave-s.png",
        "https://i.ibb.co/4MZfbQN/DALL-E-2023-03-06-12-35-09-Synthwave-Vaperwave-Cyberpunk-styled-beach-pink-sand-aqua-sky-vaperwave-s.png",
        "https://i.ibb.co/st7bn4C/DALL-E-2023-03-06-12-35-15-Synthwave-Vaperwave-Cyberpunk-styled-beach-pink-sand-aqua-sky-vaperwave-s.png",
        "https://i.ibb.co/7ncbzLh/DALL-E-2023-03-06-12-35-21-Synthwave-Vaperwave-Cyberpunk-styled-beach-pink-sand-aqua-sky-vaperwave-s.png",
        "https://i.ibb.co/DQRxf86/DALL-E-2023-08-17-10-14-06-beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/jvgYRbq/DALL-E-2023-08-17-10-14-03-beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/g3vrc79/DALL-E-2023-08-17-10-12-50-beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/b7vtZfK/DALL-E-2023-08-17-10-12-31-beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/KVcR3c4/DALL-E-2023-08-17-10-12-27-beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/Bq1MsDL/DALL-E-2023-08-17-10-12-25-beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/TPQNMLG/DALL-E-2023-08-17-10-12-01-beach-in-vaperwave-style-realistic.png",
        "https://i.ibb.co/tPCzNPr/DALL-E-2023-08-17-10-11-55-beach-in-vaperwave-style-realistic.png"
    ];    

    let mut rng = rand::rngs::StdRng::from_entropy();
    if let Some(random_url) = image_urls.choose(&mut rng) {
        // Create an embedded message
        let mut embed = CreateEmbed::default();
        embed.title("Here's a AI Generated beach image!");
        embed.image(*random_url);
        embed.color(EMBED_COLOR); 
        // footer that says requested by user and displays their profile picture
        embed.footer(|f| {
            f.text(format!("Requested by {}", msg.author.name));
            f.icon_url(msg.author.face());
            f
        });

        // Send the embedded message
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.clone_from(&embed); // Clone the embed into the closure
                e
            })
        }).await?;
    } else {
        msg.reply(&ctx.http, "No images found.").await?;
    }

    Ok(())
}