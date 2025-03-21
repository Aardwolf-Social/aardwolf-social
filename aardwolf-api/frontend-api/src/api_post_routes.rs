//-
// This is a starter for the Aardwolf Frontend API
//

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn get_posts() -> impl Responder {
    // Logic to retrieve a list of posts (replace this with actual implementation)
    let posts = vec!["Post 1", "Post 2", "Post 3"]; // Sample posts

    HttpResponse::Ok().json(posts)
}
async fn create_post() -> impl Responder {
    // Logic to create a new post (replace this with actual implementation)
    let post = "New Post"; // Sample post

    HttpResponse::Created().json(post)
}
async fn update_post() -> impl Responder {
    // Logic to update an existing post (replace this with actual implementation)
    let post = "Updated Post"; // Sample post

    HttpResponse::Ok().json(post)
}
async fn delete_post() -> impl Responder {
    // Logic to delete a post (replace this with actual implementation)
    HttpResponse::NoContent().finish()
}
async fn like_post() -> impl Responder {
    // Logic to like a post (replace this with actual implementation)
    HttpResponse::NoContent().finish()
}
async fn comment_post() -> impl Responder {
    // Logic to comment on a post (replace this with actual implementation)
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/posts", web::get().to(get_posts))
            .route("/api/posts/create", web::get().to(get_posts))
            .route("/api/posts/update/:postId", web::get().to(get_posts))
            .route("/api/posts/delete/:postId", web::get().to(get_posts))
            .route("/api/posts/like/:postId", web::get().to(get_posts))
            .route("/api/posts/comment/:postId", web::get().to(get_posts))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
