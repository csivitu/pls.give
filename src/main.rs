#[macro_use]
extern crate dotenv_codegen;

fn main() {
    // let DISCORD_TOKEN = get_env("DISCORD_TOKEN");
    let discord_token = dotenv!("DISCORD_TOKEN");
    println!("{}", discord_token);
    
    
}
