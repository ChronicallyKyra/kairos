use tokio;
use dotenv::dotenv;
use std::env::var;

mod kairos;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_token = var("DISCORD_TOKEN").expect("Discord Token");

}
