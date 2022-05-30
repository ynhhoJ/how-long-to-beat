mod types;
mod variables;

use teloxide::{adaptors::throttle::Limits, prelude::*, repl};
use types::bot::BotWithAutoSendAndThrottle;
use variables::TELOXIDE_TOKEN;

#[tokio::main]
async fn main() {
    let bot = Bot::new(TELOXIDE_TOKEN.as_str())
        .throttle(Limits::default())
        .auto_send();

    repl(
        bot,
        |message: Message, bot: BotWithAutoSendAndThrottle| async move {
            let chat_id = message.chat.id;

            bot.send_message(chat_id, "Hello world!").await?;

            respond(())
        },
    )
    .await;
}
