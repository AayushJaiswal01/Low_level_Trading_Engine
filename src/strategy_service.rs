use std::sync::Arc;
use log::info;
use crate::market_data::{MarketDataService, MarketDataError};
use crate::order_service::{OrderService, OrderError};

pub struct StrategyService {
    market_service: Arc<MarketDataService>,
    order_service: Arc<OrderService>,
}

impl StrategyService {
    pub fn new(market_service: Arc<MarketDataService>, order_service: Arc<OrderService>) -> Self {
        println!("Creating new StrategyService");
        StrategyService {
            market_service,
            order_service,
        }
    }

    pub async fn run_price_threshold_strategy(&self, symbol: &str, threshold: f64, quantity: i32) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running price threshold strategy for {} (threshold: ${})", symbol, threshold);
        let quote = self.market_service.get_quote(symbol).await?;
        println!("Current price of {}: ${}", symbol, quote.price);

        if quote.price < threshold {
            println!("Price below threshold, placing buy order");
            self.order_service.place_order(symbol.to_string(), quantity, quote.price, true)?;
            println!("Strategy executed: Bought {} shares of {} at ${}", quantity, symbol, quote.price);
        } else {
            println!("Price above threshold, no order placed");
        }

        self.order_service.show_orders()?;
        Ok(())
    }
}