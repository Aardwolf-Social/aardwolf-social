// frontend-api/src/routes/routes.rs
pub mod routes {
    pub fn get_routes() -> Vec<actix_web::Route> {
        vec![
            actix_web::Route::new("/api/users/:userId/posts")
                .method(actix_web::http::Method::GET)
                .to("get_user_posts"),
            actix_web::Route::new("/api/users/:userId/followers")
                .method(actix_web::http::Method::GET)
                .to("get_user_followers"),
            actix_web::Route::new("/api/users/follow/:userId")
                .method(actix_web::http::Method::GET)
                .to("follow_user"),
            actix_web::Route::new("/api/users/unfollow/:userId")
                .method(actix_web::http::Method::GET)
                .to("unfollow_user"),
        ]
    }
}
