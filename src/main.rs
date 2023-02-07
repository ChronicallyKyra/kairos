use tokio;
use dotenv::dotenv;
use std::env::var;

mod kairos;
mod convert;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_token = var("DISCORD_TOKEN").expect("Discord Token");


    let bot = kairos::Kairos::new(bot_token);
    bot.connect().await;
}
