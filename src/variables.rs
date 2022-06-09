use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;
use std::sync::Arc;

pub static TELOXIDE_TOKEN: Lazy<Arc<String>> = Lazy::new(|| {
    dotenv().ok();

    let teloxide_token = env::var("TELOXIDE_TOKEN");
    let token = match teloxide_token {
        Ok(token) => token,
        Err(error) => panic!("{}", error),
    };

    if token.is_empty() {
        panic!("TELOXIDE_TOKEN cannot be empty");
    }

    Arc::new(token)
});

pub static HOW_LONG_TO_BEAT_WEBSITE: Lazy<Arc<String>> = Lazy::new(|| {
    dotenv().ok();

    let how_long_to_beat_website_result = env::var("HOW_LONG_TO_BEAT_WEBSITE");
    let how_long_to_beat_website = match how_long_to_beat_website_result {
        Ok(how_long_to_beat_website) => how_long_to_beat_website,
        Err(error) => panic!("{}", error),
    };

    if how_long_to_beat_website.is_empty() {
        panic!("HOW_LONG_TO_BEAT_WEBSITE cannot be empty");
    }

    Arc::new(how_long_to_beat_website)
});
