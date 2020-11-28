use crate::ShardManagerContainer;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::fs::File;
use std::io::prelude::*;

use tokio::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn read_env() -> Result<String> {
    let mut file = File::open("./.env").expect("Unable to open .env file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the env file");
    Ok(contents)
}

#[command]
#[description = "Shutdown the bot"]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "Shutting down!").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "There was a problem getting the shard manager")
            .await?;
        return Ok(());
    }

    Ok(())
}
#[command]
#[description = "Returns env file contents as a DM"]
pub async fn envs(ctx: &Context, msg: &Message) -> CommandResult {
    let env_future = task::spawn(read_env());
    let env_string = format!("env file is \n```{}```", env_future.await??);

    let dm = msg.author.direct_message(&ctx, |m| {
        m.content(&env_string)
    })
    .await;

    match dm {
        Ok(_) => {
            let _ = msg.react(&ctx, '👌').await;
        },
        Err(why) => {
            println!("Err sending envs: {:?}", why);
            let _ = msg.reply(&ctx, "There was an error DMing you help.").await;
        },
    };
    Ok(())
}
