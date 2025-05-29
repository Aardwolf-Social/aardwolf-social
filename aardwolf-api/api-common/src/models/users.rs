// users.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub struct UserData {
    pub id: i32,
    pub name: String,
    pub email: String,
}
