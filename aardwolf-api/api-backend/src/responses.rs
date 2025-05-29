// responses.rs
pub trait Response {
    fn into_response(self) -> Self;
}

#[derive(Debug)]
pub(crate) struct ErrorResponse {
    pub(crate) message: String,
}

impl std::error::Error for ErrorResponse {}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
