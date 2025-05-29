// backend-api/src/routes/posts.rs
use aardwolf_api_common::models::posts::PostImpl;
use aardwolf_api_common::Utc;

/// Creates a new post.
pub async fn create_post(post: PostImpl) -> Result<PostImpl, String> {
    Ok(PostImpl {
        id: 1,
        title: post.title,
        content: post.content,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    })
}

/// Retrieves all posts.
pub async fn get_posts() -> Result<Vec<PostImpl>, String> {
    Ok(vec![PostImpl {
        id: 1,
        title: "Post 1".to_string(),
        content: "This is post 1".to_string(),
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    }])
}
