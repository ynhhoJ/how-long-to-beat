use teloxide::{
    adaptors::{AutoSend, Throttle},
    Bot,
};

pub type BotWithAutoSendAndThrottle = AutoSend<Throttle<Bot>>;
