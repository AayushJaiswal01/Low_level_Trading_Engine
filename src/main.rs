mod market_data;
mod order_service;
mod strategy_service;

use dotenv::dotenv;
use env_logger;
use log::info;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    println!("Starting trading engine");

    let market_service = Arc::new(market_data::MarketDataService::new());
    let order_service = Arc::new(order_service::OrderService::new());
    let strategy_service = strategy_service::StrategyService::new(
        Arc::clone(&market_service),
        Arc::clone(&order_service),
    );

    println!("Running strategy for AAPL");
    strategy_service.run_price_threshold_strategy("AAPL", 220.0, 10).await?;

    Ok(())
}