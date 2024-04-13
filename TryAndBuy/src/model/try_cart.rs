use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct TryCart {
    pub try_cart_id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub added_at: String,
}