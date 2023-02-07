use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::Message;
// use serenity::framework::standard::macros::{group, hook};
use serenity::framework::standard::StandardFramework;
use regex::Regex;

use crate::convert::{self, DiscordTimestamp};

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Connected to user {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, mut message: Message) {
        println!("{} sent a message in channel {}", message.author, message.channel(&ctx).await.expect("Channel from cache"));

        let content = &message.content;

        let reg = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("Valid regex");
        if let Some(matches) = Regex::find(&reg, content) {
            match convert::parse(&matches.as_str()) {
                Some(timestamp) => {
                    // Timestamp always has to be datetime, so if date can't be parsed, current date should be inferred
                    // and vice versa
                    // Insert into message
    
                    let stamp = timestamp.short_date();

                    let edited = reg.replace(content, stamp).to_string();
    
                    message.edit(ctx, | m | m.content(edited)).await.expect("Message edited");
                }
                _ => {}
            }
        }
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