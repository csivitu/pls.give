use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use isahc::prelude::*;
use std::time::Duration;

use url::Url;

use tokio::task;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn get_link(link: String) -> Result<String> {

    let url_parse = Url::parse(&link);
    
    if url_parse.is_err() {
        return Ok(String::from("Invalid URL format"))
    }

    let mut response = Request::post("http://gg.gg/create")
        .timeout(Duration::from_secs(5))
        .body(format!(r#"long_url={}"#, link))?
        .send()?;
    let resp = response.text()?;
    println!("Response is :|{}|", resp);
    Ok(resp)
}

#[command]
#[description = "URL shortener"]
#[description = "Usage: pls.give link <link>"]
pub async fn link(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let say_content = if args.is_empty() {
        "Invalid format. The format is \npls.give link `some-long-link`".to_string()
    } else {
        let resp1 = task::spawn(get_link(args.rest().to_string()));

        format!("Link is \n{}.", resp1.await??)
    };
    msg.reply(&ctx.http, say_content).await?;
    Ok(())
}
