use serde::Serialize;

pub struct ErrorResponder {}

impl ErrorResponder {
    pub fn internal_server_error(&self) -> Error {
        Error {
            code: 500,
            message: String::from("Internal Server Error"),
        }
    }
}

#[derive(Serialize)]
pub struct Error {
    code: u16,
    message: String,
}
