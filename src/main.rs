use std::thread;

mod telegram_bot;
mod discord_bot;
mod url_storage;
mod url;

fn main() {
    pretty_env_logger::init();

    let telegram_thread = thread::spawn(|| {
        telegram_bot::start();
    });
    let discord_thred = thread::spawn(|| {
        discord_bot::start();
    });
    telegram_thread.join().unwrap();
    discord_thred.join().unwrap();
}

const REDIRECT_BUTTON_TEXT: &str = "Contact";

fn generate_bot_notification_text(group_name: Option<&str>) -> String {
    format!(
"Hey there👋
It's important to make sure everyone on {} is aware of potential scams. So, here's an official reminder for you: NO ADMIN WILL EVER PM YOU FIRST. You can open a ticket to reach one of the available team member for any assistance or concern.
When you leave a complain in the group, it might attract scammers and open you to potential risks. Let's work together to keep those scammers at bay.
You can always come back to this message to open a ticket.

Click on the below button to chat with an Admin. 👇👇👇",
        group_name.unwrap_or("this group")
    )
}

fn contains_keyword(text: &str) -> bool {
    text.to_lowercase().split(" ").into_iter().any(|item| KEYWORDS.contains(&item))
}

const KEYWORDS: &'static [&'static str] = &[
    "admin",
    "please",
    "pls",
    "help",
    "token",
    "wallet",
    "missing",
    "import",
    "address",
    "metamask",
    "trust wallet",
    "extension",
    "migrate",
    "v2",
    "bridge",
    "buy",
    "gas",
    "price",
    "contract",
    "staking",
    "stake",
    "seed phrase",
    "keychain",
    "private key",
    "withdraw",
    "slippage",
    "swap",
    "liquidity",
    "unstake",
    "transaction",
    "snapshot",
    "exchange",
    "airdrop",
    "eligible",
    "eligibility",
    "contract",
    "support",
    "network",
    "chain",
    "binance",
    "v1",
    "undelegated",
    "delegated",
    "rewards",
    "reward",
    "validator",
    "bridging",
    "fee",
    "need",
    "how",
    "use",
    "provide",
    "transaction",
    "hash",
    "deposit",
    "compromised",
    "swiped",
    "swipe",
    "pool",
    "farm",
    "farms",
    "pair",
    "v3",
    "connected",
    "connect",
    "connection",
    "ledger",
    "web3",
    "connecting",
    "instructions",
    "instruction",
    "transfer",
    "key",
    "software",
    "hardware",
    "bitbox",
    "account",
    "backup",
    "file",
    "device",
    "switch",
    "disappeared",
    "compatible",
    "backup",
    "unconfirmed",
    "lost",
    "increase",
    "app",
    "ethereum",
    "eth",
    "bnb",
    "bitcoin",
    "btc",
    "erc20",
    "password",
    "error",
    "question",
    "whirlpool",
    "node",
    "firmware",
    "coldcard",
    "trezor",
    "usb"
];

