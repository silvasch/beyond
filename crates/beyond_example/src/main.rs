use std::process::ExitCode;

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
        let hostname = String::from_utf8_lossy(
            &std::process::Command::new("hostname")
                .output()
                .unwrap()
                .stdout,
        )
        .trim()
        .to_string();

        HelloResponse {
            message: format!(
                "Hello, {}! This message was generated on '{}'.",
                request.name, hostname
            ),
        }
    }
}

fn main() -> ExitCode {
    let server_impl = ServerImpl;
    if let Some(exit_code) = Beyond::run_server(server_impl) {
        return exit_code;
    }

    let name = match std::env::args().nth(2) {
        Some(name) => name,
        None => {
            eprintln!("usage: beyond_example <destination> <name>");
            return ExitCode::FAILURE;
        }
    };
    let destination = std::env::args()
        .nth(1)
        .expect("if element two exists, element one should always exist");

    let client = Beyond::new_client(destination, "beyond_example".to_string());
    let response = match client.hello(HelloRequest { name }) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::FAILURE;
        }
    };
    println!("{}", response.message);

    ExitCode::SUCCESS
}
