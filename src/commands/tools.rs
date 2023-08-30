use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::builder::CreateEmbed;
use base64::{encode, decode};
use serenity::framework::standard::CommandError;
use hex::{encode as hex_encode, decode as hex_decode};
use anyhow::Result;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;
use crate::config::EMBED_COLOR;


#[group]
#[commands(base64, hex, binary, reverse, qr)] 
struct Tools;

#[command]
#[description("Encodes or decodes text using Base64")]
#[usage("rbase64 <encode/decode> <text>")]
async fn base64(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let operation = match args.single::<String>() {
        Ok(op) => op,
        Err(_) => {
            return Err(CommandError::from("Invalid usage. Use `encode` or `decode`"));
        }
    };
    let text = args.rest();

    match operation.as_str() {
        "encode" => {
            let encoded = encode(text);

            let mut embed = CreateEmbed::default();
            embed.title("Base64 Encode");
            embed.description(format!("Encoded text:\n```{}\n```", encoded));
            embed.color(EMBED_COLOR);
            embed.footer(|f| {
                f.text(format!("Requested by {}", msg.author.name));
                f.icon_url(msg.author.face());
                f
            });

            msg.channel_id.send_message(ctx, |m| m.set_embed(embed)).await?;
        }
        "decode" => {
            let decoded_bytes = decode(text);

            let decoded_text = match decoded_bytes {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes).into_owned();
                    text
                }
                Err(_) => {
                    return Err(CommandError::from("Failed to decode Base64 data"));
                }
            };

            let mut embed = CreateEmbed::default();
            embed.title("Base64 Decode");
            embed.description(format!("Decoded text:\n```{}\n```", decoded_text));
            embed.color(EMBED_COLOR);
            embed.footer(|f| {
                f.text(format!("Requested by {}", msg.author.name));
                f.icon_url(msg.author.face());
                f
            });

            msg.channel_id.send_message(ctx, |m| m.set_embed(embed)).await?;
        }
        _ => {
            return Err(CommandError::from("Invalid operation. Use `encode` or `decode`"));
        }
    }

    Ok(())
}

#[command]
#[description("Encodes or decodes text using hexadecimal")]
#[usage("rhex <encode/decode> <text>")]
async fn hex(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let operation = match args.single::<String>() {
        Ok(op) => op,
        Err(_) => {
            return Err(CommandError::from("Invalid usage. Use `encode` or `decode`"));
        }
    };
    let text = args.rest();

    match operation.as_str() {
        "encode" => {
            let encoded = hex_encode(text);

            let mut embed = CreateEmbed::default();
            embed.title("Hex Encode");
            embed.description(format!("Encoded text:\n```{}\n```", encoded));
            embed.color(EMBED_COLOR);
            embed.footer(|f| {
                f.text(format!("Requested by {}", msg.author.name));
                f.icon_url(msg.author.face());
                f
            });

            msg.channel_id.send_message(ctx, |m| m.set_embed(embed)).await?;
        }
        "decode" => {
            let decoded_bytes = hex_decode(text);

            let decoded_text = match decoded_bytes {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes).into_owned();
                    text
                }
                Err(_) => {
                    return Err(CommandError::from("Failed to decode hexadecimal data"));
                }
            };

            let mut embed = CreateEmbed::default();
            embed.title("Hex Decode");
            embed.description(format!("Decoded text:\n```{}\n```", decoded_text));
            embed.color(EMBED_COLOR);
            embed.footer(|f| {
                f.text(format!("Requested by {}", msg.author.name));
                f.icon_url(msg.author.face());
                f
            });

            msg.channel_id.send_message(ctx, |m| m.set_embed(embed)).await?;
        }
        _ => {
            return Err(CommandError::from("Invalid operation. Use `encode` or `decode`"));
        }
    }

    Ok(())
}

#[command]
#[description("Encodes or decodes text using binary")]
#[usage("rbinary <encode/decode> <text>")]
async fn binary(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let operation = match args.single::<String>() {
        Ok(op) => op,
        Err(_) => {
            return Err(CommandError::from("Invalid usage. Use `encode` or `decode`"));
        }
    };
    let text = args.rest();

    match operation.as_str() {
        "encode" => {
            let encoded = text
                .chars()
                .map(|c| format!("{:08b}", c as u8))
                .collect::<Vec<String>>()
                .join(" ");

            let mut embed = CreateEmbed::default();
            embed.title("Binary Encode");
            embed.description(format!("Encoded text:\n```\n{}\n```", encoded));
            embed.color(EMBED_COLOR);
            embed.footer(|f| {
                f.text(format!("Requested by {}", msg.author.name));
                f.icon_url(msg.author.face());
                f
            });

            msg.channel_id.send_message(ctx, |m| m.set_embed(embed)).await?;
        }
        "decode" => {
            let decoded_text = text
                .split_whitespace()
                .map(|bin| u8::from_str_radix(bin, 2).map(|c| c as char))
                .collect::<Result<Vec<_>, _>>()
                .map(|chars| chars.into_iter().collect::<String>())
                .map_err(|_| CommandError::from("Failed to decode binary data"))?;

            let mut embed = CreateEmbed::default();
            embed.title("Binary Decode");
            embed.description(format!("Decoded text:\n```{}\n```", decoded_text));
            embed.color(EMBED_COLOR);
            embed.footer(|f| {
                f.text(format!("Requested by {}", msg.author.name));
                f.icon_url(msg.author.face());
                f
            });

            msg.channel_id.send_message(ctx, |m| m.set_embed(embed)).await?;
        }
        _ => {
            return Err(CommandError::from("Invalid operation. Use `encode` or `decode`"));
        }
    }

    Ok(())
}

#[command]
#[description("Reverses the input text")]
#[usage("rreverse <text>")]
async fn reverse(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = args.rest();

    let reversed_text = text.chars().rev().collect::<String>();

    let mut embed = CreateEmbed::default();
    embed.title("Text Reverse");
    embed.description(format!("Reversed text:\n```\n{}\n```", reversed_text));
    embed.color(EMBED_COLOR);
    embed.footer(|f| {
        f.text(format!("Requested by {}", msg.author.name));
        f.icon_url(msg.author.face());
        f
    });

    msg.channel_id.send_message(ctx, |m| m.set_embed(embed)).await?;

    Ok(())
}

#[command]
#[description("Generates a QR code from the given content")]
#[usage("rqr <content>")]
async fn qr(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let content = args.rest();

    let qrcode = QRBuilder::new(content)
        .build()
        .map_err(|_| CommandError::from("Failed to generate QR code"))?;

    let img = ImageBuilder::default()
        .shape(Shape::RoundedSquare)
        .background_color([255, 255, 255, 255]) // Handles transparency
        .fit_width(600)
        .to_bytes(&qrcode)
        .map_err(|_| CommandError::from("Failed to generate QR image"))?;

    msg.channel_id
        .send_files(&ctx.http, vec![(img.as_slice(), "qrcode.png")], |m| {
            m.content(format!("QR Code for: {}", content))
        })
        .await?;

    Ok(())
}
