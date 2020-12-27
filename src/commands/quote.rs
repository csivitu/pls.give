use serde_json::Value;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

// use tokio::task;
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// async fn get_quote() -> Result<HashMap<String,String>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;

//     Ok(resp)
// }

// #[command]
// #[description = "Get a random quote"]
// #[description = "Usage: pls.give quote "]
// #[description = "Uses the \"They Said So\" Quotes API"]
// pub async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
//     let resp1 = task::spawn(get_quote()).await??;
//     println!("{}",resp1.find_equiv(&"success"));
//     let say_content = format!("Meeting Link is \n{}.", format!("{:#?}", resp1));
//     msg.reply(&ctx.http, say_content).await?;
//     Ok(())
// }
