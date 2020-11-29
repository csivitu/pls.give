mod commands;

use std::{collections::HashSet, env, sync::Arc};

use dotenv::dotenv;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{
        event::ResumedEvent,
        gateway::{Activity, Ready},
        user::OnlineStatus,
    },
    prelude::*,
};

use commands::{admin::*, help::*, link::*, misc::*, pastebin::*, witeboard::*};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_presence(
            Some(Activity::playing("pls.give help")),
            OnlineStatus::Online,
        )
        .await;
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[group]
#[owners_only]
#[prefix = "admin"]
#[description = "Administration commands which only bot owners are allowed to use"]
#[commands(quit, envs)]
struct Admin;

#[group]
#[description = "Miscelenaous commands"]
#[commands(ping)]
struct Misc;

#[group]
#[description = "Collaboration commands"]
#[commands(board)]
struct Collaboration;

#[group]
#[description = "Utility commands"]
#[commands(link, paste)]
struct Utility;

#[tokio::main]
async fn main() {
    if let Err(_) = dotenv() {
        println!("Warning ! Cannot find .env file");
    };
    
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");
    
    env::var("PASTEBIN_TOKEN").expect("Expected PASTEBIN_TOKEN in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            println!(
                "Setting owner to {} (ID: {})",
                info.owner.name, info.owner.id
            );
            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("pls.give "))
        .help(&HELP)
        .group(&ADMIN_GROUP)
        .group(&UTILITY_GROUP)
        .group(&COLLABORATION_GROUP)
        .group(&MISC_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        println!("Client  error: {:?}", why);
    }
}
