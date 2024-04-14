use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct Address {
    pub address_id: Option<i32>,
    pub user_id: i32,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub add_state: String,
    pub postal_code: i32,
    pub country: String,
}
