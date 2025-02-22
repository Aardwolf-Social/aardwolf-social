//-
// This is a starter for the Aardwolf Frontend API
//

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn get_users() -> impl Responder {
    // Logic to retrieve a list of users (replace this with actual implementation)
    let users = vec!["User 1", "User 2", "User 3"]; // Sample users

    HttpResponse::Ok().json(users)
}
async fn get_followers() -> impl Responder {
    // Logic to retrieve followers of a user (replace this with actual implementation)
    let followers = vec!["Follower 1", "Follower 2", "Follower 3"]; // Sample followers

    HttpResponse::Ok().json(followers)
}
async fn follow_user() -> impl Responder {
    // Logic to follow a user (replace this with actual implementation)
    let message = "User followed successfully"; // Sample message

    HttpResponse::Ok().json(message)
}
async fn unfollow_user() -> impl Responder {
    // Logic to unfollow a user (replace this with actual implementation)
    let message = "User unfollowed successfully"; // Sample message

    HttpResponse::Ok().json(message)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/users/:userId/posts", web::get().to())
            .route("/api/users/:userId/followers", web::get().to())
            .route("/api/users/follow/:userId", web::get().to())
            .route("/api/users/unfollow/:userId", web::get().to())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}