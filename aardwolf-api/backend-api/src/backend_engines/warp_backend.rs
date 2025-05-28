// aardwolf-api/backend-engines/warp_backend.rs
use crate::responses::ErrorResponse;
use warp::reject::Reject;

impl Reject for ErrorResponse {}

async fn create_post_warp() {
    todo!()
}

async fn get_posts_warp() {
    todo!()
}

pub fn routes() {
    todo!()
}

#[tokio::main]
async fn main() {
    todo!()
}
