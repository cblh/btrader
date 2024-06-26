use btrader::binance_interface::BinanceAccount;
use btrader::telegram_interface::TelegramBotInterface;
use binance::model::{FillInfo, Order, Transaction};

pub struct MockBinanceAccount;

impl BinanceAccount for MockBinanceAccount {
    fn market_buy(&self, _symbol: String, _quantity: f64) -> Result<Transaction, String> {
        Ok(Transaction {
            symbol: String::from("BTC-USD"),
            order_id: 123456789,
            client_order_id: String::from("client-order-001"),
            transact_time: 1624694768,
            price: 35000.0,
            orig_qty: 1.5,
            executed_qty: 1.0,
            cummulative_quote_qty: 35000.0,
            status: String::from("filled"),
            time_in_force: String::from("GTC"),
            side: String::from("buy"),
            fills: vec![
                FillInfo {
                    price: 35000.0,
                    qty: 1.0,
                    commission: 0.005,
                    commission_asset: String::from("USD"),
                    trade_id: Some(987654321),
                }
            ],
        })
    }

    fn market_sell(&self, _symbol: String, _quantity: f64) -> Result<Transaction, String> {
        Ok(Transaction {
            symbol: String::from("BTC-USD"),
            order_id: 123456789,
            client_order_id: String::from("client-order-001"),
            transact_time: 1624694768,
            price: 35000.0,
            orig_qty: 1.5,
            executed_qty: 1.0,
            cummulative_quote_qty: 35000.0,
            status: String::from("filled"),
            time_in_force: String::from("GTC"),
            side: String::from("buy"),
            fills: vec![
                FillInfo {
                    price: 35000.0,
                    qty: 1.0,
                    commission: 0.005,
                    commission_asset: String::from("USD"),
                    trade_id: Some(987654321),
                }
            ],
        })
    }

    fn order_status(&self, _symbol: String, _order_id: u64) -> Result<Order, String> {
        Ok(Order {
            symbol: String::from("BTC-USD"),
            order_id: 987654321,
            client_order_id: String::from("client-order-002"),
            price: 36000.0,
            orig_qty: String::from("1.5"),
            executed_qty: String::from("1.0"),
            status: String::from("filled"),
            time_in_force: String::from("GTC"),
            type_name: String::from("limit"),
            side: String::from("buy"),
            stop_price: 0.0,
            iceberg_qty: String::from("0.0"),
            time: 1624764768,
        })
    }
}

pub struct MockTelegramBot;

impl TelegramBotInterface for MockTelegramBot {
    fn start(&self) {
        // Mock implementation: Do nothing
    }

    fn send_message(&self, _message: String) {
        // Mock implementation: Do nothing
    }
}
