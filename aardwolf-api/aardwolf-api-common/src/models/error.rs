// error.rs
use serde::{Deserialize, Serialize};

pub trait Error {
    fn get_message(&self) -> String;
    fn get_code(&self) -> i32;
}

#[derive(Serialize, Deserialize)]
pub struct ErrorImpl {
    pub message: String,
    pub code: i32,
}

impl Error for ErrorImpl {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_code(&self) -> i32 {
        self.code
    }
}
