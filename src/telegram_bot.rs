use log::warn;
use reqwest::Url;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup}, utils::command::BotCommands};

use crate::{contains_keyword, generate_bot_notification_text, url::{check_url_string, url_from_chat_id, CANT_PARSE_URL_ERROR, SUCCESSFUL_MESSAGE}, url_storage::UrlStorage, REDIRECT_BUTTON_TEXT};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

const TELOXIDE_TOKEN: &str = "6896702664:AAGD_wkHv7fPg0HCMVezDyIuHl60g4cjylw";

const ADMIN_IDS: [u64; 2] = [
    5488031843, 7041070645
];

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "set a url for current chat/group")]
    SetUrl(String),
    #[command(description = "set a default url, bot will send it if URL wasn't set for a chat/group")]
    SetDefaultUrl(String)
}

#[tokio::main]
pub async fn start() {
    let bot = Bot::new(TELOXIDE_TOKEN);

    Dispatcher::builder(
        bot,
        Update::filter_message()
        .branch(
            // Filter a maintainer by a user ID.
            dptree::filter(|msg: Message| {
                msg.from().map(|user| ADMIN_IDS.contains(&user.id.0)).unwrap_or_default()
            })
            .filter_command::<Command>()
            .endpoint(handle_commands),
        )
        .endpoint(check_on_keywords)
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn check_on_keywords(bot: Bot, msg: Message) -> HandlerResult {
    if let Some(text) = msg.text() {
        if contains_keyword(text) {
            let url = url_from_chat_id(msg.chat.id);

            match url {
                Some(url) => {
                    bot.send_message(
                        msg.chat.id,
                        generate_bot_notification_text(
                            msg.chat.title()
                        )
                    )
                    .reply_markup(InlineKeyboardMarkup::new([
                        [InlineKeyboardButton::url(
                            REDIRECT_BUTTON_TEXT,
                            url
                        )]
                    ]))
                    .await?;
                },
                None => {
                    warn!("Something was wrong...");
                },
            }
        }
    }
    Ok(())
}

async fn handle_commands(bot: Bot, msg: Message, cmd: Command) -> HandlerResult {
    match cmd {
        Command::SetUrl(url) => {
            if check_url_string(&url) {
                UrlStorage::set_url(
                    msg.chat.id.0.to_string(),
                    url
                );
                bot.send_message(msg.chat.id, SUCCESSFUL_MESSAGE).await?;
            } else {
                bot.send_message(msg.chat.id, CANT_PARSE_URL_ERROR).await?;
            }
        },
        Command::SetDefaultUrl(url) => {
            if check_url_string(&url) {
                UrlStorage::set_default_url(
                    url
                );
                bot.send_message(msg.chat.id, SUCCESSFUL_MESSAGE).await?;
            } else {
                bot.send_message(msg.chat.id, CANT_PARSE_URL_ERROR).await?;
            }
        },
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
    };
    Ok(())
}