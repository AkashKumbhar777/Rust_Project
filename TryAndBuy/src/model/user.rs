use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct User {
    pub user_id: Option<i32,
    pub login_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub profile_picture: Option<Vec<u8>>,
    pub created_at: String,
    pub updated_at: String,
}