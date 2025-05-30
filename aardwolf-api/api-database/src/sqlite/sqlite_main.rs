// aardwolf-api/api-database/src/sqlite/sqlite_main.rs
use crate::database_main::DatabaseConnection;
use crate::traits::db_handler::DbHandler;
use aardwolf_api_common::models::posts::PostImpl;
use aardwolf_api_common::models::error::ErrorImpl;

pub struct MySqlHandler {
    conn: DatabaseConnection,
}

impl MySqlHandler {
    pub fn new(conn: DatabaseConnection) -> Self {
        MySqlHandler { conn }
    }
}

impl DbHandler for MySqlHandler {
    type PostError = ErrorImpl;

    fn get_posts(&self) -> Result<Vec<PostImpl>, ErrorImpl> {
        let query = "SELECT * FROM posts";
        match self.conn.query(query) {
            Ok(rows) => {
                let mut posts = Vec::new();
                for row in rows {
                    let post = PostImpl {
                        id: row.get("id"),
                        title: row.get("title"),
                        content: row.get("content"),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                        author: row.get("author"),
                    };
                    posts.push(post);
                }
                Ok(posts)
            }
            Err(err) => Err(ErrorImpl::new(format!("Failed to retrieve posts: {}", err))),
        }
    }

    fn create_post(&self, post: PostImpl) -> Result<PostImpl, ErrorImpl> {
        if post.title.is_empty() || post.content.is_empty() {
            return Err(ErrorImpl::new("Title and content are required".to_string()));
        }

        match self.conn.execute("INSERT INTO posts (title, content) VALUES (?, ?)", &[&post.title, &post.content]) {
            Ok(_) => Ok(post),
            Err(err) => Err(ErrorImpl::new(format!("Failed to create post: {}", err))),
        }
    }

    fn update_post(&self, post_id: i32, post: PostImpl) -> Result<PostImpl, ErrorImpl> {
        if post.title.is_empty() || post.content.is_empty() {
            return Err(ErrorImpl::new("Title and content are required".to_string()));
        }

        // Check if post ID is valid (i.e., greater than 0)
        if post_id <= 0 {
            return Err(ErrorImpl::new("Invalid post ID".to_string()));
        }

        // Check if post exists in the database
        let query = format!("SELECT * FROM posts WHERE id = {}", post_id);
        match self.conn.query(query) {
            Ok(rows) => {
                if rows.is_empty() {
                    return Err(ErrorImpl::new(format!("Post with ID {} does not exist", post_id)));
                }
            }
            Err(err) => {
                return Err(ErrorImpl::new(format!("Failed to check if post exists: {}", err)));
            }
        }

        // Update the post
        match self.conn.execute("UPDATE posts SET title = ?, content = ? WHERE id = ?", &[&post.title, &post.content, &post_id]) {
            Ok(_) => Ok(post),
            Err(err) => Err(ErrorImpl::new(format!("Failed to update post: {}", err))),
        }
    }

    fn delete_post(&self, post_id: i32) -> Result<(), ErrorImpl> {
        // Check if post ID is valid (i.e., greater than 0)
        if post_id <= 0 {
            return Err(ErrorImpl::new("Invalid post ID".to_string()));
        }

        // Check if post exists in the database
        let query = format!("SELECT * FROM posts WHERE id = {}", post_id);
        match self.conn.query(query) {
            Ok(rows) => {
                if rows.is_empty() {
                    return Err(ErrorImpl::new(format!("Post with ID {} does not exist", post_id)));
                }
            }
            Err(err) => {
                return Err(ErrorImpl::new(format!("Failed to check if post exists: {}", err)));
            }
        }

        // Delete the post
        let query = format!("DELETE FROM posts WHERE id = {}", post_id);
        match self.conn.execute(query) {
            Ok(_) => Ok(()), // Return Ok(()) to indicate success
            Err(err) => Err(ErrorImpl::new(format!("Failed to delete post: {}", err))),
        }
    }

    fn like_post(&self, post_id: i32) -> Result<(), ErrorImpl> {
        // Check if post ID is valid (i.e., greater than 0)
        if post_id <= 0 {
            return Err(ErrorImpl::new("Invalid post ID".to_string()));
        }

        // Check if post exists in the database
        let query = format!("SELECT * FROM posts WHERE id = {}", post_id);
        match self.conn.query(query) {
            Ok(rows) => {
                if rows.is_empty() {
                    return Err(ErrorImpl::new(format!("Post with ID {} does not exist", post_id)));
                }
            }
            Err(err) => {
                return Err(ErrorImpl::new(format!("Failed to check if post exists: {}", err)));
            }
        }

        // Increment like count for the post
        let query = format!("UPDATE posts SET likes = likes + 1 WHERE id = {}", post_id);
        match self.conn.execute(query) {
            Ok(_) => Ok(()),
            Err(err) => Err(ErrorImpl::new(format!("Failed to like post: {}", err))),
        }
    }

}
