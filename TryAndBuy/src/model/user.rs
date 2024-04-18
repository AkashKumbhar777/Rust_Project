use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct User {
    pub user_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: Vec<String>,
    pub phone: String,
    pub profile_picture: Option<Vec<u8>>,
    pub user_role: Option<String>,
    pub created_at: String,
<<<<<<< HEAD
    pub updated_at: Option<String>,
=======
    pub updated_at: String,
    pub user_role:String
>>>>>>> shreya
}