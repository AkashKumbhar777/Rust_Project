use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct SalesHistory {
    pub sale_id: i32,
    pub order_id: i32,
    pub quantity: i32,
    pub sale_date: String,
}
