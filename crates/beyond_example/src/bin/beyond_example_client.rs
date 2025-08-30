use beyond_example::{Beyond, Request};

fn main() {
    let name = match std::env::args().nth(2) {
        Some(name) => name,
        None => {
            eprintln!("usage: beyond_example_client <destination>");
            return;
        }
    };
    let destination = std::env::args().nth(1).unwrap();

    let client = Beyond::new_client(destination, "beyond_example_server".to_string());

    println!("{}", client.hello(Request { name }).message);
}
