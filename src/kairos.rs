use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::Message;
// use serenity::framework::standard::macros::{group, hook};
use serenity::framework::standard::StandardFramework;

use crate::convert;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Connected to user {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        println!("{} sent a message in channel {}", message.author, message.channel(&ctx).await.expect("Channel from cache"));

        let content = message.content;

        // Match to regex??
        // Determine if is a specific date, or if is time of current date
    }
}



pub struct Kairos {
    discord_token: String,
}

impl Kairos {
    pub fn new(token: String) -> Kairos {
        Kairos { discord_token: token }
    }
    pub async fn connect(&self) {
        let framework = StandardFramework::new()
            .configure(|c| c.prefix("~"));
    
        
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(&self.discord_token, intents)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client");
    
            if let Err(why) = &mut client.start().await {
                println!("An error occurred while running the client: {:?}", why);
            }
    }
}