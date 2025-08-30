use std::process::ExitCode;

fn main() -> ExitCode {
    let name = match std::env::args().nth(2) {
        Some(name) => name,
        None => {
            eprintln!("usage: beyond_example_client <destination> <name>");
            return ExitCode::FAILURE;
        }
    };
    let destination = std::env::args().nth(1).expect("if element two exists, element one should always exist");

    let client = beyond_example::Beyond::new_client(destination, "beyond_example_server".to_string());
    let response = client.hello(beyond_example::HelloRequest { name });
    println!("{}", response.message);

    ExitCode::SUCCESS
}
