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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/db/users", web::get().to(get_posts))
            .route("/api/db/users", web::post().to(create_post))
            .route("/api/db/users/:userId/posts", web::get().to(get_posts))
            .route("/api/db/posts", web::get().to(get_posts))
            .route("/api/db/posts", web::post().to(create_post))
            .route("/api/db/posts/:postId", web::put().to(update_post))
            .route("/api/db/posts/:postId", web::delete().to(delete_post))
            .route("/api/db/posts/:postId/like", web::post().to(like_post))
            .route("/api/db/posts/:postId/comment", web::post().to(comment_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}