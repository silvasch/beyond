use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    pub message: String,
}

#[derive(beyond::Beyond)]
#[beyond_route(hello Request Response)]
pub struct InternalBeyond;

impl InternalBeyond {
    pub fn hello(&mut self, request: Request) -> Response {
        Response {
            message: format!("Hello, {}!", request.name),
        }
    }
}
