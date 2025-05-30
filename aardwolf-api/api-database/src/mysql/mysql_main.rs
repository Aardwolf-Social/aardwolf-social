// aardwolf-api/api-database/src/mysql/mysql_main.rs
use crate::database_main::DatabaseConnectionManager;
use crate::traits::db_handler::DbHandler;
use aardwolf_api_common::models::error::ErrorImpl;
use aardwolf_api_common::models::posts::PostImpl;
use aardwolf_models::schema::posts::dsl::*;
use diesel::prelude::*;

pub struct MySqlHandler {
    connection: DatabaseConnectionManager,
}

impl MySqlHandler {
    pub fn new(connection: DatabaseConnectionManager) -> Self {
        MySqlHandler { connection }
    }
}

impl DbHandler for MySqlHandler {
    type PostError = ErrorImpl;

    fn get_posts(&self) -> Result<Vec<PostImpl>, ErrorImpl> {
        let mut conn = self.connection.get()?;
        posts
            .load::<PostImpl>(&mut conn)
            .map_err(|err| ErrorImpl::new(format!("Failed to retrieve posts: {}", err)))
    }

    fn create_post(&self, post: PostImpl) -> Result<PostImpl, ErrorImpl> {
        use crate::schema::posts;

        if post.title.is_empty() || post.content.is_empty() {
            return Err(ErrorImpl::new("Title and content are required".to_string()));
        }

        let conn = self.connection.get()?;
        diesel::insert_into(posts::table)
            .values(&post)
            .execute(&mut conn)
            .map(|_| post)
            .map_err(|err| ErrorImpl::new(format!("Failed to create post: {}", err)))
    }

    fn update_post(&self, post_id: i32, post: PostImpl) -> Result<PostImpl, ErrorImpl> {
        use crate::schema::posts::dsl::*;

        if post.title.is_empty() || post.content.is_empty() {
            return Err(ErrorImpl::new("Title and content are required".to_string()));
        }

        if post_id <= 0 {
            return Err(ErrorImpl::new("Invalid post ID".to_string()));
        }

        let conn = self.connection.get()?;
        diesel::update(posts.find(post_id))
            .set((title.eq(post.title), content.eq(post.content)))
            .execute(&mut conn)
            .map(|_| post)
            .map_err(|err| ErrorImpl::new(format!("Failed to update post: {}", err)))
    }

    fn delete_post(&self, post_id: i32) -> Result<(), ErrorImpl> {
        use crate::schema::posts::dsl::*;

        if post_id <= 0 {
            return Err(ErrorImpl::new("Invalid post ID".to_string()));
        }

        let conn = self.connection.get()?;
        diesel::delete(posts.find(post_id))
            .execute(&conn)
            .map(|_| ())
            .map_err(|err| ErrorImpl::new(format!("Failed to delete post: {}", err)))
    }

    fn like_post(&self, post_id: i32) -> Result<(), ErrorImpl> {
        use crate::schema::posts::dsl::*;

        if post_id <= 0 {
            return Err(ErrorImpl::new("Invalid post ID".to_string()));
        }

        let conn = self.connection.get()?;
        diesel::update(posts.find(post_id))
            .set(likes.eq(likes + 1))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(|err| ErrorImpl::new(format!("Failed to like post: {}", err)))
    }
}
