#[cfg(test)]
mod mocks;
mod tests {
    use crate::mocks::MockBinanceAccount;
    use crate::mocks::MockTelegramBot;
    use btrader::config::Configuration;
    use btrader::calculation_cluster::CalculationCluster;
    use btrader::depth_cache_interface::LocalOrderBook;
    use btrader::triangular_relationship::TriangularRelationship;
    use btrader::trading_pair::TradingPair;
    use btrader::depth_cache_interface::MockDepthCacheInterface;
    use std::collections::HashMap;

    fn create_test_config() -> Configuration {
        Configuration::new("config/sample_config.json")
    }

    fn create_test_depth_cache() -> MockDepthCacheInterface {
        let mut mock = MockDepthCacheInterface::new();
        mock.expect_get_depth()
        .return_const(LocalOrderBook {
            first_event: true,
            last_update_id: 123456,
            event_time: 1234567890,
            bids: vec![],
            asks: vec![],
        });
        mock
    }

    fn create_test_relationships() -> HashMap<String, TriangularRelationship> {
        let mut relationships = HashMap::new();
        let base = "BTC".to_string();
        let start = TradingPair::new("BTCETH".to_string(), "BTC".to_string(), "ETH".to_string(), 0.01); 
        let middle = TradingPair::new("ETHLTC".to_string(), "ETH".to_string(), "LTC".to_string(), 0.01);
        let end = TradingPair::new("LTCBTC".to_string(), "LTC".to_string(), "BTC".to_string(), 0.01);
        relationships.insert(
            "test_relationship".to_string(),
            TriangularRelationship::new(base, start, middle, end),
        );
        relationships
    }

    #[test]
    fn test_calculation_relationship_profit_threshold() {
        let relationships = create_test_relationships();
        let depth_cache = Box::new(create_test_depth_cache());
        let config = create_test_config();
        let account = Box::new(MockBinanceAccount);
        let bot = Box::new(MockTelegramBot);

        let cluster = CalculationCluster::new(relationships.clone(), depth_cache, config.clone(), account, bot);
        let relationship = relationships.get("test_relationship").unwrap().clone();

        let deal = cluster.calculate_relationship(relationship);

        assert!(deal.get_profit() == -1.0);
    }
}
