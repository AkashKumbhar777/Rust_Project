use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct RefundReturn {
    pub refund_id: Option<i32>,
    pub order_id: i32,
    pub reason: String,
    pub refunded_amount: f64,
    pub refund_date: String,
    pub payment_id : Option<String>
}