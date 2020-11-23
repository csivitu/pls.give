mod commands;

use std::{
    env,
    collections::HashSet,
    sync::Arc,
};

use dotenv::dotenv;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    model::{event::ResumedEvent, gateway::Ready},
    http::Http,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    prelude::*,
};

use commands::{
    ping::*,
    owner::*,
};


struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _:Context, _:ResumedEvent) {
        println!("Resumed");
    }
}


#[group]
#[commands(ping, quit)]
struct General;


#[tokio::main]
async fn main() {
    dotenv()
        .expect(".env file missing!");
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in the environment");


    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            println!("Setting owner to {} (ID: {})", info.owner.name, info.owner.id);
            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                   .owners(owners)
                   .prefix("pls.give "))
        .group(&GENERAL_GROUP);

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
        tokio::signal::ctrl_c().await.expect("Could not register ctrl c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        println!("Client  error: {:?}", why);
    }
}
