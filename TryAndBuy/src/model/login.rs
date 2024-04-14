use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct Login {
    pub login_id: Option<i32>,
    pub username: String,
    pub password: String,
    pub user_role: String,
    pub last_logged_in: String,
}
