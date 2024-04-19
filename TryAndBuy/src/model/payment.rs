use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub reciept_no: Option<i32>,
    pub user_id: Option<i32>,
    pub order_id: Option<String>,
    pub payment_id: Option<String>,
    pub currency: Option<String>,
    pub total_amount: Option<f64>,
    pub payment_status: Option<String>,
}
