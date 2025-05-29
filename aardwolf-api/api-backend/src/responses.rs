// api-backend/src/responses.rs
pub trait Response {
    fn into_response(self) -> Self;
}

#[derive(Debug)]
pub struct ErrorResponse {
    pub(crate) message: String,
}

impl std::error::Error for ErrorResponse {}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ErrorResponse {
    #[allow(dead_code)]
    pub(crate) fn new(message: String) -> Self {
        ErrorResponse { message }
    }
}

pub type Result<T> = std::result::Result<T, ErrorResponse>;