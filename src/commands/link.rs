use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use tokio::task;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn get_link(link: String) -> Result<String> {
    let response = reqwest::Client::new()
        .post("http://gg.gg/create")
        .form(&[("long_url", link)])
        .send()
        .await?;
    Ok(response.text().await?)
}

#[command]
#[description = "URL shortener"]
pub async fn link(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let say_content = if args.is_empty() {
        "Invalid format. The format is \npls.give link `some-long-link`".to_string()
    } else {
        let resp1 = task::spawn(get_link(args.rest().to_string()));

        let short_link = resp1.await??;
        format!("Link is \n{}.", short_link)
    };
    msg.reply(&ctx.http, say_content).await?;
    Ok(())
}
