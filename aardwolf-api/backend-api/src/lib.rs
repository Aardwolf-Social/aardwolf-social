<<<<<<< HEAD

// API backend modules
mod api_post_routes;
=======
// backend-api/src/lib.rs
pub mod backend_engines {
    pub mod actix_backend;
    pub mod actix_responses;
    pub mod warp_backend;
}
pub mod routes {
    pub mod posts;
}
pub use routes::posts; // Allows calling `crate::posts` directly
pub mod responses;
>>>>>>> d1b24ddd5f52965698d08e47af9333a61cbcc526
