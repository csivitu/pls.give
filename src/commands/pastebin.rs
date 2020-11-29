use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

use tokio::task;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn pastebin_post(text: String, token: String) -> Result<String> {
    let params = [
        ("api_option", "paste"),
        ("api_paste_code", &text),
        ("api_dev_key", &token),
    ];
    let response = reqwest::Client::new()
        .post("https://pastebin.com/api/api_post.php")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;
    Ok(response.text().await?)
}

#[command]
#[description = "Uploads text to pastebin and returns link"]
#[description = "Usage: pls.give paste <link>"]
pub async fn paste(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let pastebin_token =
        env::var("PASTEBIN_TOKEN").expect("Expected PASTEBIN_TOKEN in the environment");

    let say_content = if args.is_empty() {
        "Invalid format. The format is \npls.give paste `some text`".to_string()
    } else {
        let resp1 = task::spawn(pastebin_post(
            args.rest().to_string(),
            pastebin_token.to_string(),
        ));

        format!("Pastebin link is \n{}", resp1.await??)
    };
    msg.reply(&ctx.http, say_content).await?;
    Ok(())
}
