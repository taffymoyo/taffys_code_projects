use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use rand::Rng;
use chrono;

mod leetcode;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        match msg.content.as_str() {
            "!ping" => {
                msg.channel_id.say(&ctx.http, "pong!").await.unwrap();
            }
            "!roll" => {
                let roll = rand::thread_rng().gen_range(1..=100);
                msg.channel_id.say(&ctx.http, format!("🎲 You rolled a {}!", roll)).await.unwrap();
            }
            _ if msg.content.starts_with("!solved") => {
                let parts: Vec<&str> = msg.content.splitn(2, ' ').collect();
                if parts.len() < 2 {
                    msg.channel_id.say(&ctx.http, "Please provide a problem slug e.g. !solved two-sum").await.unwrap();
                    return;
                }
                let slug = parts[1];
                match leetcode::get_problem(slug).await {
                    Some((title, difficulty)) => {
                        let now = chrono::Local::now();
                        let date = now.format("%d/%m/%Y");
                        let message = format!(
                            "Daily leetcode question for {}! \n\nDifficulty: {}\n{}\n\n@everyone share your answers in <#1483367237896245259>",
                            date, difficulty, title
                        );
                        msg.channel_id.say(&ctx.http, message).await.unwrap();
                    }
                    None => { msg.channel_id.say(&ctx.http, "Couldn't find that problem!").await.unwrap(); }
                }
            }
            _ => {
                println!("Not a valid command");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("token not found");
    
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    client.start().await.expect("Error starting client");
}