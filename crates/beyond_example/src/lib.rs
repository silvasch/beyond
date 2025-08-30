use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct HelloResponse {
    pub message: String,
}

#[derive(beyond::Beyond)]
#[beyond_route(hello HelloRequest HelloResponse)]
pub struct ServerImpl;

impl ServerImpl {
    pub fn hello(&self, request: HelloRequest) -> HelloResponse {
        HelloResponse {
            message: format!("Hello, {}!", request.name),
        }
    }
}
