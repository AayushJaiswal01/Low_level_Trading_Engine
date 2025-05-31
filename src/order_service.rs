use std::sync::{Arc, Mutex};
use log::info;
use thiserror::Error;

// Custom error type for order operations
#[derive(Error, Debug)]
pub enum OrderError {
    #[error("Invalid order: {0}")]
    InvalidOrder(String),
}

// Represents a single order
#[derive(Clone, Debug)]
pub struct Order {
    pub symbol: String,
    pub quantity: i32,
    pub price: f64,
    pub is_buy: bool,
}

pub struct OrderService {
    orders: Arc<Mutex<Vec<Order>>>,
    next_id:Arc<Mutex<u64>>,
}

impl OrderService {
    ///Creates a new OrderService with an empty order list
    pub fn new() -> Self {
        info!("Creating new OrderService");
        OrderService {
            orders: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)), 
        }
    }

    pub fn place_order(&self, symbol: String, quantity: i32, price: f64, is_buy: bool) -> Result<u64, OrderError> {
        if symbol.trim().is_empty() {
            return Err(OrderError::InvalidOrder("Symbol cannot be empty".to_string()));
        }
        if quantity <= 0 {
            return Err(OrderError::InvalidOrder("Quantity must be positive".to_string()));
        }
        if price <= 0.0 {
            return Err(OrderError::InvalidOrder("Price must be positive".to_string()));
        }

        let id = {
            let mut next_id = self.next_id.lock().map_err(|e| {
                OrderError::InvalidOrder(format!("Failed to lock ID: {}", e))
            })?;
            let current_id = *next_id;
            *next_id += 1;
            current_id
        };

        let order = Order {
            symbol: symbol.clone(),
            quantity,
            price,
            is_buy,
        };
        let mut orders = self.orders.lock().map_err(|e| {
            OrderError::InvalidOrder(format!("Failed to lock orders: {}", e))
        })?;
        orders.push(order.clone());
        info!(
            "Placed {} order [ID {}]: {} shares of {} at ${}",
            if is_buy { "buy" } else { "sell" },
            id, quantity, symbol, price
        );
        Ok(id)
    }

    pub fn show_orders(&self) -> Result<(), OrderError> {
        let orders = self.orders.lock().map_err(|e| {
            OrderError::InvalidOrder(format!("Failed to lock orders: {}", e))
        })?;
        if orders.is_empty() {
            info!("No orders placed yet");
        } else {
            info!("Current orders ({} total):", orders.len());
            for (i, order) in orders.iter().enumerate() {
                let total = order.quantity as f64 * order.price;
                info!(
                    "Order {}: {} {} shares of {} at ${} (Total: ${})",
                    i + 1,
                    if order.is_buy { "Buy" } else { "Sell" },
                    order.quantity,
                    order.symbol,
                    order.price,
                    total
                );
            }
        }
        Ok(())
    }
}