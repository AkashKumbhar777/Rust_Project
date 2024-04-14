use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct BuyCart {
    pub buy_cart_id: Option<i32>,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub total_amount: f64,
}

// -- buy_cart table:
// CREATE TABLE buy_cart (
//     buy_cart_id SERIAL PRIMARY KEY,
//     user_id INT  REFERENCES user_table(user_id),
//     product_id INT REFERENCES product(product_id),
//     quantity INT  DEFAULT 0,
//     total_amount NUMERIC(10, 2) DEFAULT 0.00
// );