use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;


#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct Product {
    pub product_id: i32,
    pub product_name: String,
    pub product_description: Option<String>,
    pub price: f64,
    pub image_url: Option<String>,
    pub specifications: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}