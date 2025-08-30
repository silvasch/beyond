pub struct HelloRequest {
    pub name: String,
}

pub struct HelloResponse {
    pub message: String,
}

#[derive(beyond::Beyond)]
#[beyond_route(hello HelloRequest HelloResponse)]
pub struct Internal;

impl Internal {
    pub fn hello(&self, request: HelloRequest) -> HelloResponse {
        HelloResponse {
            message: format!("Hello, {}!", request.name),
        }
    }
}
