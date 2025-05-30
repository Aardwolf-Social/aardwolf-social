// api-database/src/traits/db_handler.rs
use aardwolf_api_common::models::error::{Error, ErrorImpl};
use aardwolf_api_common::models::posts::PostImpl;

pub trait DbHandler {
    type PostError: Error;

    fn get_posts(&self) -> Result<Vec<PostImpl>, ErrorImpl>;
    fn create_post(&self, post: PostImpl) -> Result<PostImpl, ErrorImpl>;
    fn update_post(&self, post_id: i32, post: PostImpl) -> Result<PostImpl, ErrorImpl>;
    fn delete_post(&self, post_id: i32) -> Result<(), ErrorImpl>;
    fn like_post(&self, post_id: i32) -> Result<(), ErrorImpl>;
}