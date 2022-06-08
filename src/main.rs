mod handlers;
mod parser;
mod types;
mod utils;
mod variables;

use handlers::inline_query::inline_query;
use teloxide::{adaptors::throttle::Limits, prelude::*};
use variables::TELOXIDE_TOKEN;

#[tokio::main]
async fn main() {
    let bot = Bot::new(TELOXIDE_TOKEN.as_str())
        .throttle(Limits::default())
        .auto_send();

    let handler = inline_query().await;

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}
