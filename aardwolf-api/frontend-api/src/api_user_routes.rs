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