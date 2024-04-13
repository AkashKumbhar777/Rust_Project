use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundReturn {
    pub refund_id: i32,
    pub order_id: i32,
    pub reason: String,
    pub refunded_amount: f64,
    pub refund_date: String,
}