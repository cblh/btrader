use binance::model::{Transaction, Order};
pub trait BinanceAccount {
    fn market_buy(&self, symbol: String, quantity: f64) -> Result<Transaction, String>;
    fn market_sell(&self, symbol: String, quantity: f64) -> Result<Transaction, String>;
    fn order_status(&self, symbol: String, order_id: u64) -> Result<Order, String>;
}