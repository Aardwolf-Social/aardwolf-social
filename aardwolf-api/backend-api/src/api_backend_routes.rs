//-
// This is a starter for the Aardwolf Frontend API
//

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
async fn auth_login() -> impl Responder {
    // Logic to handle user authentication (replace this with actual implementation)
    HttpResponse::Ok().json("User authenticated")
}
async fn users() -> impl Responder {
    // Logic to retrieve a list of users (replace this with actual implementation)
    HttpResponse::Ok().json("List of users")
}
async fn get_user() -> impl Responder {
    // Logic to retrieve a specific user (replace this with actual implementation)
    HttpResponse::Ok().json("User details")
}
async fn update_user() -> impl Responder {
    // Logic to update a user (replace this with actual implementation)
    HttpResponse::Ok().json("User updated")
}
async fn delete_user() -> impl Responder {
    // Logic to delete a user (replace this with actual implementation)
    HttpResponse::Ok().json("User deleted")
}
async fn posts() -> impl Responder {
    // Logic to retrieve a list of posts (replace this with actual implementation)
    HttpResponse::Ok().json("List of posts")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/auth/login", web::get().to())
            .route("/api/users", web::get().to())
            .route("/api/users/:userId", web::get().to())
            .route("/api/users/:userId", web::put().to())
            .route("/api/users/:userId", web::delete().to())
            .route("/api/posts", web::get().to())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}