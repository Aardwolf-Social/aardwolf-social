<<<<<<< HEAD
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
=======
// aardwolf-api/frontend-api/src/api_user_routes.rs

pub trait ApiUserRoutes {
    fn get_users_route(&self) -> Box<dyn RouteHandler>;
    fn get_user_posts_route(&self) -> Box<dyn RouteHandler>;
    fn get_user_followers_route(&self) -> Box<dyn RouteHandler>;
    fn follow_user_route(&self) -> Box<dyn RouteHandler>;
    fn unfollow_user_route(&self) -> Box<dyn RouteHandler>;
}

pub trait RouteHandler {
    fn handle(&self, request: &Request) -> Response;
}

// Define a struct to hold the routes
pub struct ApiUserRoutesImpl {
    pub get_users: Box<dyn RouteHandler>,
    pub get_user_posts: Box<dyn RouteHandler>,
    pub get_user_followers: Box<dyn RouteHandler>,
    pub follow_user: Box<dyn RouteHandler>,
    pub unfollow_user: Box<dyn RouteHandler>,
}

impl ApiUserRoutes for ApiUserRoutesImpl {
    fn get_users_route(&self) -> Box<dyn RouteHandler> {
        self.get_users.clone()
    }

    fn get_user_posts_route(&self) -> Box<dyn RouteHandler> {
        self.get_user_posts.clone()
    }

    fn get_user_followers_route(&self) -> Box<dyn RouteHandler> {
        self.get_user_followers.clone()
    }

    fn follow_user_route(&self) -> Box<dyn RouteHandler> {
        self.follow_user.clone()
    }

    fn unfollow_user_route(&self) -> Box<dyn RouteHandler> {
        self.unfollow_user.clone()
    }
}

// Define the main function to create the HTTP server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let routes = ApiUserRoutesImpl {
        get_users: Box::new(handlers::GetUsersHandler),
        get_user_posts: Box::new(handlers::GetUserPostsHandler),
        get_user_followers: Box
>>>>>>> d1b24ddd5f52965698d08e47af9333a61cbcc526
