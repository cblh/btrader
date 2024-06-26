// src/binance_account.rs
use binance::account::*;
use binance::api::*;
use binance::model::*;
use crate::binance_interface::BinanceAccount;

pub struct RealBinanceAccount {
    account: Account,
}

impl RealBinanceAccount {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        Self {
            account: Binance::new(api_key, api_secret),
        }
    }
}

impl BinanceAccount for RealBinanceAccount {
    fn market_buy(&self, symbol: String, quantity: f64) -> Result<Transaction, String> {
        self.account.market_buy(symbol, quantity).map_err(|e| e.to_string())
    }

    fn market_sell(&self, symbol: String, quantity: f64) -> Result<Transaction, String> {
        self.account.market_sell(symbol, quantity).map_err(|e| e.to_string())
    }
    fn order_status(&self, symbol: String, order_id: u64) -> Result<Order, String>{
        self.account.order_status(symbol, order_id).map_err(|e| e.to_string())
    }
}
