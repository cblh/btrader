
// src/telegram_bot.rs
use crate::telegram::TelegramBot;
use crate::telegram_interface::TelegramBotInterface;
use crate::config::Configuration;

pub struct RealTelegramBot {
    bot: TelegramBot,
}

impl RealTelegramBot {
    pub fn new(config: Configuration) -> Self {
        Self {
            bot: TelegramBot::new(config),
        }
    }
}

impl TelegramBotInterface for RealTelegramBot {
    fn start(&self) {
        self.bot.start();
    }

    fn send_message(&self, message: String) {
        self.bot.send_message(message);
    }
}
