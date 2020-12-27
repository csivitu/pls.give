use serde_json::Value;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use tokio::task;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn get_meeting_room() -> Result<String> {
    // let dyte_str = String::from("{{\"operationName\":\"CreateSession\",\"variables\":{{\"roomName\":\"{}\",\"input\":{{\"password\":null,\"title\":\"test\",\"toggles\":{{\"waitingRoom\":false}}}}}},\"query\":\"mutation CreateSession($roomName: String!, $input: CreateSessionInput!) {{\n  createSession(roomName: $roomName, input: $input) {{\n    roomNodeLink\n    roomName\n    __typename\n  }}\n}}\n\"}}");
    let data = r#"
    {"operationName":"CreateSession",
    "variables":{"roomName":"literate-ping",
    "input":{"password":null,"title":"","toggles":{"waitingRoom":false}}},
    "query":"mutation CreateSession($roomName: String!, $input: CreateSessionInput!) {\n  createSession(roomName: $roomName, input: $input) {\n    roomNodeLink\n    roomName\n    __typename\n  }\n}\n"}
    "#;

    let response = reqwest::Client::new()
        .post("https://api.node.dyte.in/graphql")
        .header("content-type","application/json")
        .header("Host", "api.node.dyte.in")
        .header("authorization", "Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6ImYwMzUzNmRmLTA4ZWUtNDdlOC05YzZhLTg4YTNlNGU0OTEyYSIsImxvZ2dlZEluIjpmYWxzZSwiaWF0IjoxNjA4NDA2NzA5LCJleHAiOjE2MDg0OTMxMDl9.sHAZnvyU9Vt-CJnfh1oU00vIuUqueGxzywUAcGZIyva3srWrN1lw-saFXjNiq49Ueid9yQUgqtJnzlUoOxs156aR0ugaAiGSeHnsMjz_sOIosPZmxxUILL_ZiKsbzHby4O0FYbeo5KhBIw-T3PgSZlRHt17j7haTyQoraWZLrGu49z7A_YvNzEGemWHBD2SJQYH0tocaUNY7IZA1rb4CmYXqNsG3WzRW6geJuqO-592dY8XaY6tMxmQrPbdmxwWtxcWrdy_Ul-A06wTblremy482Nnic2q9Di7wcNp2m2t11si6MejMitp2lVuqXIpJ-L_5yb-S-C4tI0FQAGP3TyQ")
        .json(&map)
        .send()
        .await?;
    Ok(response.text().await?)
}

#[command]
#[description = "URL shortener"]
#[description = "Usage: pls.give link <link>"]
pub async fn dyte(ctx: &Context, msg: &Message) -> CommandResult {
    let resp1 = task::spawn(get_meeting_room());
    let say_content = format!("Meeting Link is \n{}.", resp1.await??);
    msg.reply(&ctx.http, say_content).await?;
    Ok(())
}
