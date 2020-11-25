use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use uuid::Uuid;

#[command]
#[description = "Whiteboard"]
#[description = "gives link to whiteboard"]
#[description = "Usage: pls.give board"]
pub async fn board(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let say_content = format!("https://witeboard.com/{}.", Uuid::new_v4());
    msg.reply(&ctx.http, say_content).await?;
    Ok(())
}
