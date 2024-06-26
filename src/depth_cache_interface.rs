use mockall::automock;
#[automock]
pub trait DepthCacheInterface {
    fn get_depth(&self, symbol: &str) -> LocalOrderBook;
}

use binance::model::*;

#[derive(Debug, Clone)]
pub struct LocalOrderBook {
  pub first_event: bool,
  pub last_update_id: u64,
  pub event_time: u64,
  pub bids: Vec<Bids>,
  pub asks: Vec<Asks>,
}


use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::RwLock;
use std::sync::{Arc, Mutex};
#[derive(Debug)]
pub struct DepthCache {
  pub map: Arc<RwLock<HashMap<String, LocalOrderBook>>>,
  pub in_tx: Mutex<Sender<String>>,
  pub out_rx: Mutex<Receiver<LocalOrderBook>>,
}