use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub login_id: i32,
    pub username: String,
    pub password: String,
    pub user_role: String,
    pub last_logged_in: String,
}
