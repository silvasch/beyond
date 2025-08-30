use std::process::ExitCode;

fn main() -> ExitCode {
    let server_impl = beyond_example::ServerImpl;
    if let Some(exit_code) = beyond_example::Beyond::run_server(server_impl) {
        return exit_code;
    }

    let name = match std::env::args().nth(2) {
        Some(name) => name,
        None => {
            eprintln!("usage: beyond_example <destination> <name>");
            return ExitCode::FAILURE;
        }
    };
    let destination = std::env::args().nth(1).expect("if element two exists, element one should always exist");

    let client = beyond_example::Beyond::new_client(destination, "beyond_example".to_string());
    let response = match client.hello(beyond_example::HelloRequest { name }) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::FAILURE;
        }
    };
    println!("{}", response.message);

    ExitCode::SUCCESS
}

