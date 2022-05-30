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
