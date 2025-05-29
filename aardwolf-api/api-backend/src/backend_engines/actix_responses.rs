// backend-api/src/backend_engines/actix_responses.rs
use aardwolf_api_common::models::direct_messages::PrivateMessageReply as CommonPrivateMessageReply;
use actix_web::{Responder, HttpResponse, HttpRequest};
use actix_web::body::BoxBody;

pub struct ActixPrivateMessageReply(pub CommonPrivateMessageReply);

impl Responder for ActixPrivateMessageReply {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().json(&self.0 .0) // unpack twice!
    }
}
