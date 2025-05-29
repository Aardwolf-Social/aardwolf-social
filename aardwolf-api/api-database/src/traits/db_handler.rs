// database-api/src/traits/db_handler.rs
extern crate aardwolf_api_common;
use aardwolf_api_common::models::error::Error;
use aardwolf_api_common::models::posts::Post;

pub trait DbHandler {
    fn get_posts(&self) -> Result<Vec<Box<dyn Post>>, Box<dyn Error>>;
    fn create_post(&self, post: Box<dyn Post>) -> Result<(), Box<dyn Error>>;
    fn update_post(&self, post_id: i32, post: Box<dyn Post>) -> Result<(), Box<dyn Error>>;
    fn delete_post(&self, post_id: i32) -> Result<(), Box<dyn Error>>;
    fn like_post(&self, post_id: i32) -> Result<(), Box<dyn Error>>;
}
