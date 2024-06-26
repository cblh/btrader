pub trait TelegramBotInterface {
    fn start(&self);
    fn send_message(&self, message: String);
}