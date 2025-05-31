use reqwest::Client;
use serde::Deserialize;
use std::env;
use log::error;
use thiserror::Error;
use serde_with::{serde_as,DisplayFromStr};

#[derive(Error, Debug)]
pub enum MarketDataError {
    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Failed to parse JSON: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}
#[serde_as]
#[derive(Deserialize, Debug)]
pub struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    pub symbol: String,
    #[serde(rename = "05. price")]
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
}

pub struct MarketDataService {
    client: Client,
    api_key: String,
}

impl MarketDataService {
    /// Creates a new MarketDataService, loading the API key from .env
    pub fn new() -> Self {
        let api_key = env::var("ALPHA_VANTAGE_API_KEY")
            .expect("ALPHA_VANTAGE_API_KEY not set in .env");
        MarketDataService {
            client: Client::new(),
            api_key,
        }
    }

    /// Fetches the current stock quote for a given symbol from Alpha Vantage
    pub async fn get_quote(&self, symbol: &str) -> Result<GlobalQuote, MarketDataError> {
        let url = format!(
            "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            symbol, self.api_key
        );
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;
        let global_quote = data
            .get("Global Quote")
            .ok_or_else(|| MarketDataError::InvalidResponse("Missing 'Global Quote' in response".to_string()))?;
        let quote = serde_json::from_value(global_quote.clone()).map_err(|e| {
            error!("Failed to parse quote for {}: {}", symbol, e);
            MarketDataError::Parse(e)
        })?;
        Ok(quote)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_quote() {

        let service = MarketDataService::new();
        let result = service.get_quote("AAPL").await;
        assert!(result.is_ok(), "Failed to fetch quote: {:?}", result.err());
        let quote = result.unwrap();
        assert_eq!(quote.symbol, "AAPL", "Expected symbol to be AAPL");
        assert!(quote.price > 0.0, "Expected price to be positive");
    }
}