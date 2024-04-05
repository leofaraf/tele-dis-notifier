use reqwest::Url;
use teloxide::types::ChatId;

use crate::url_storage::UrlStorage;


pub const SUCCESSFUL_MESSAGE: &str = "Successful";

pub const CANT_PARSE_URL_ERROR: &str = "Can't parse message. Please, check the URL format. Example of format:
https://example.com/";

pub fn url_from_chat_id(chat_id: ChatId) -> Option<Url> {
    Url::parse(
        UrlStorage::get_url(chat_id.0.to_string()).as_str()
    ).ok()
}

pub fn check_url_string(url: &str) -> bool {
    Url::parse(
        url
    ).is_ok()
}