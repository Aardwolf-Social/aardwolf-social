// frontend-api/src/lib.rs
mod api_post_routes;
mod api_user_routes;

pub mod routes {
    pub mod actix_routes;
}

pub mod handlers {
    pub mod actix_handlers;
}