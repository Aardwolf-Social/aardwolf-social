// api-backend/src/lib.rs
pub mod backend_engines {
    pub mod actix_backend;
    pub mod actix_responses;
}
pub mod routes {
    pub mod posts;
}
pub use routes::posts; // Allows calling `crate::posts` directly
pub mod responses;