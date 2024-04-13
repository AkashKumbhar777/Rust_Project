use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyCart {
    pub buy_cart_id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub total_amount: f64,
}