use crate::config::Configuration;
use crate::depth_cache::DepthCache;
use crate::triangular_relationship::TriangularRelationship;
use console::style;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct CalculationCluster {
  relationships: HashMap<String, TriangularRelationship>,
  depth_cache: DepthCache,
  config: Configuration,
}

impl CalculationCluster {
  pub fn new(
    relationships: HashMap<String, TriangularRelationship>,
    depth_cache: DepthCache,
    config: Configuration,
  ) -> CalculationCluster {
    CalculationCluster {
      relationships: relationships,
      depth_cache: depth_cache,
      config: config,
    }
  }
  pub fn start(&self) -> () {
    let relationships = self.relationships.clone();
    let relationships_names: Vec<String> = self.relationships.keys().cloned().collect();
    loop {
      relationships_names.iter().for_each(|rel| {
        // println!("------------------------------------------------------------------------------------------------");
        let deal = self.calculate_relationship(relationships.get(rel).unwrap().clone());
        if (deal.get_profit() >= self.config.trading_profit_threshold)
          && ((self.get_epoch_ms() - deal.get_timestamp()) <= self.config.trading_age_threshold)
        {
          println!(
            "[{}] Deal: {:?}...",
            style(format!("{:+.3}%", deal.get_profit())).bold().dim(),
            deal.get_actions()
          );
        }
      })
    }
  }
  fn get_epoch_ms(&self) -> u64 {
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_millis() as u64
  }
  fn correct_quantity(&self, quantity: f64, step: f64) -> f64 {
    (quantity / step).floor() * step
  }
  fn calculate_relationship(&self, relationship: TriangularRelationship) -> Deal {
    let pairs = relationship.get_trading_pairs();
    let pair_names = relationship.get_pairs();
    let pair_actions = relationship.get_actions();
    let fee_multiplier = ((100.0 - self.config.trading_taker_fee) / 100.0).powi(3);
    let mut lowest_timestamp: u64 = u64::MAX;
    let mut timestamp: u64;
    let mut profit: f64;
    let mut best_profit: f64 = -1.0;
    let mut current_quantity: f64;
    let mut helper_quantity: f64;
    let mut tmp_quantity: f64;
    let mut results = Deal::new();
    let mut tmp_deal: Deal;

    // Iterate over investment values
    let min_investment = (self.config.investment_min / self.config.investment_step) as i32;
    let max_investment = (self.config.investment_max / self.config.investment_step) as i32;
    for investment in (min_investment..=max_investment).step_by(1) {
      let true_investment = investment as f64 * self.config.investment_step;
      current_quantity = true_investment;
      // println!("-----------------------------------------");
      // println!("Initial: {}BTC", current_quantity);
      tmp_deal = Deal::new();
      for (j, pair_name) in pair_names.iter().enumerate() {
        let depth_book = self.depth_cache.get_depth(&pair_name);
        timestamp = depth_book.event_time;
        if timestamp < lowest_timestamp {
          lowest_timestamp = timestamp;
        }
        helper_quantity = current_quantity;
        current_quantity = 0.0;
        if pair_actions[j] == "BUY" {
          // Buying means diving by the price
          // When you're buying, your balance depends on the step size
          let prices = depth_book.asks;
          for ask in prices.iter() {
            // println!(
            //   "{} Order book: Price={} TotalQty={}",
            //   pair_name, ask.price, ask.qty
            // );
            tmp_quantity = self.correct_quantity(helper_quantity / ask.price, pairs[j].get_step());
            // println!("HelperQty={}, TmpQty={}", helper_quantity, tmp_quantity);
            if ask.qty >= tmp_quantity {
              current_quantity += tmp_quantity;
            // println!("Buying quota");
            // println!(
            //   "Trade #{}: {} {} for {} {} (price: {})",
            //   j + 1,
            //   pair_actions[j],
            //   helper_quantity,
            //   current_quantity,
            //   pair_name,
            //   ask.price
            // );
            // println!("--")
            } else {
              tmp_quantity = self.correct_quantity(ask.qty, pairs[j].get_step());
              current_quantity += tmp_quantity;
              // println!("Buying whole thing");
              // println!(
              //   "Trade #{}: {} {} for {} {} (price: {})",
              //   j + 1,
              //   pair_actions[j],
              //   ask.qty * ask.price,
              //   current_quantity,
              //   pair_name,
              //   ask.price
              // );
              // println!("--")
            }
            helper_quantity -= ask.qty * ask.price;
            if helper_quantity <= 0.0 {
              break;
            }
          }
          tmp_deal.add_action(pair_name.clone(), pair_actions[j].clone(), current_quantity)
        } else {
          // Selling means multiplying by the price
          tmp_deal.add_action(
            pair_name.clone(),
            pair_actions[j].clone(),
            self.correct_quantity(helper_quantity, pairs[j].get_step()),
          );
          let prices = depth_book.bids;
          for bid in prices.iter() {
            // println!(
            //   "{} Order book: Price={} TotalQty={}",
            //   pair_name, bid.price, bid.qty
            // );
            if bid.qty >= helper_quantity {
              current_quantity +=
                self.correct_quantity(helper_quantity, pairs[j].get_step()) * bid.price;
            // println!("Selling quota");
            // println!(
            //   "Trade #{}: {} {} for {} {} (price: {})",
            //   j + 1,
            //   pair_actions[j],
            //   self.correct_quantity(helper_quantity, pairs[j].get_step()),
            //   current_quantity,
            //   pair_name,
            //   bid.price
            // );
            // println!("--")
            } else {
              current_quantity += self.correct_quantity(bid.qty, pairs[j].get_step()) * bid.price;
              // println!("Selling whole thing");
              // println!(
              //   "Trade #{}: {} {} for {} {} (price: {})",
              //   j + 1,
              //   pair_actions[j],
              //   self.correct_quantity(bid.qty, pairs[j].get_step()),
              //   current_quantity,
              //   pair_name,
              //   bid.price
              // );
              // println!("--")
            }
            helper_quantity -= bid.qty;
            if helper_quantity <= 0.0 {
              break;
            }
          }
        }
      }
      // println!("Fee multiplier = {}", fee_multiplier);
      // println!("Current qty = {}", current_quantity);
      // println!(
      //   "current_quantity * fee_multiplier = {}",
      //   current_quantity * fee_multiplier
      // );
      profit = ((current_quantity * fee_multiplier) - true_investment) / true_investment;
      if profit >= best_profit {
        results = tmp_deal;
        best_profit = profit;
      }
      // println!("Profit: {:+.3}", profit * 100.0);
      // if profit >= 1.0 {
      //   for _ in 0..10 {
      //     println!(
      //       "==============================================================================="
      //     )
      //   }
      // }
    }
    results.set_profit(best_profit);
    results.set_timestamp(lowest_timestamp);
    results
  }
}

#[derive(Debug, Clone)]
struct Deal {
  profit: f64,
  timestamp: u64,
  actions: Vec<Action>,
}

impl Deal {
  pub fn new() -> Deal {
    Deal {
      profit: -1.0,
      timestamp: 0,
      actions: Vec::new(),
    }
  }
  pub fn add_action(&mut self, pair: String, action: String, quantity: f64) {
    self.actions.push(Action::new(pair, action, quantity))
  }
  pub fn get_actions(&self) -> Vec<Action> {
    self.actions.clone()
  }
  pub fn set_profit(&mut self, profit: f64) {
    self.profit = profit
  }
  pub fn get_profit(&self) -> f64 {
    self.profit
  }
  pub fn set_timestamp(&mut self, timestamp: u64) {
    self.timestamp = timestamp
  }
  pub fn get_timestamp(&self) -> u64 {
    self.timestamp
  }
}

#[derive(Debug, Clone)]
struct Action {
  pair: String,
  action: String,
  quantity: f64,
}

impl Action {
  pub fn new(pair: String, action: String, quantity: f64) -> Action {
    Action {
      pair: pair,
      action: action,
      quantity: quantity,
    }
  }
  pub fn get_pair(&self) -> String {
    self.pair.clone()
  }
  pub fn get_action(&self) -> String {
    self.action.clone()
  }
  pub fn get_quantity(&self) -> f64 {
    self.quantity
  }
}