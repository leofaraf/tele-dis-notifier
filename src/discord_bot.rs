use std::env;

use log::{info, warn};
use serenity::all::{Button, CreateButton, CreateMessage};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::{contains_keyword, generate_bot_notification_text, REDIRECT_BUTTON_TEXT};
use crate::url::{check_url_string, url_from_id, CANT_PARSE_URL_ERROR, SUCCESSFUL_MESSAGE};
use crate::url_storage::UrlStorage;

struct Handler;

const ADMIN_IDS: [u64; 1] = [
    1106200623260631153
];

fn get_id_from_message(msg: &Message) -> u64 {
    match msg.guild_id {
        Some(gid) => gid.get(),
        None => msg.channel_id.get()
    }
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event. This is called whenever a new message is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be
    // dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot && contains_keyword(&msg.content) {
            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            
            match url_from_id(get_id_from_message(&msg)) {
                Some(url) => {
                    info!("Url: {}", url);
                    let generated_text = generate_bot_notification_text(None);
                    info!("{}", generated_text);

                    msg.channel_id
                    .send_message(
                        &ctx, 
                        CreateMessage::new().content(generated_text).button(
                            CreateButton::new_link(url)
                            .label(REDIRECT_BUTTON_TEXT)
                        )
                    ).await.unwrap();
                },
                None => {
                    warn!("Something was wrong...");
                },
            }

            // if let Err(why) = msg.repl.say(
            //     &ctx.http, generate_bot_notification_text(title)
            // ).await {
            //     warn!("Error sending message: {why:?}");
            // };

        } else if ADMIN_IDS.contains(&msg.author.id.get()) {
            let v: Vec<&str> = msg.content.split(" ").collect();
            match v[..] {
                ["/seturl", url] => {
                    info!("Setting url... ({})", url);

                    if check_url_string(url) {
                        UrlStorage::set_url(
                            get_id_from_message(&msg).to_string(),
                            url.to_string()
                        );

                        if let Err(why) = msg.channel_id.say(&ctx.http, SUCCESSFUL_MESSAGE).await {
                            warn!("Error sending message: {why:?}");
                        }
                    } else {
                        if let Err(why) = msg.channel_id.say(&ctx.http, CANT_PARSE_URL_ERROR).await {
                            warn!("Error sending message: {why:?}");
                        }
                    }
                },
                ["/setdefaulturl", url] => {
                    info!("Setting default url... ({})", url);

                    if check_url_string(url) {
                        UrlStorage::set_default_url(
                            url.to_string()
                        );

                        if let Err(why) = msg.channel_id.say(&ctx.http, SUCCESSFUL_MESSAGE).await {
                            warn!("Error sending message: {why:?}");
                        }
                    } else {
                        if let Err(why) = msg.channel_id.say(&ctx.http, CANT_PARSE_URL_ERROR).await {
                            warn!("Error sending message: {why:?}");
                        }
                    }
                },
                _ => {}
            };
        };  
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
pub async fn start() {
    // Configure the client with your Discord bot token in the environment.
    let token = "MTIyNTc3NDUxNTM5MzQwMDg5Mw.GsTWz1.JgTEG66o2doDGOhamv22BfZOCu4WMEEY-gRSIQ";
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        info!("Client error: {why:?}");
    }
}