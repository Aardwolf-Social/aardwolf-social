// aardwolf-api-common/src/models/posts.rs
use serde::{Deserialize, Serialize};

pub trait Post: Serialize + Send + Sync {
    fn get_id(&self) -> i32;
    fn get_title(&self) -> &str;
    fn get_content(&self) -> &str;
    fn get_created_at(&self) -> &str;
    fn get_updated_at(&self) -> &str;
}

#[derive(Serialize, Deserialize)]
pub struct PostImpl {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Post for PostImpl {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    fn get_created_at(&self) -> &str {
        &self.created_at
    }

    fn get_updated_at(&self) -> &str {
        &self.updated_at
    }
}

pub fn get_posts() -> Vec<PostImpl> {
    vec![PostImpl {
        id: 1,
        title: "Example Post".to_string(),
        content: "Hello, world!".to_string(),
        created_at: "2025-03-07T12:00:00Z".to_string(),
        updated_at: "2025-03-07T12:30:00Z".to_string(),
    }]
}
