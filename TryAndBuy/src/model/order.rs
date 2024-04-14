use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct Order {
    pub order_id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub order_status: String,
    pub order_date: String,
    pub total_amount: f64,
}