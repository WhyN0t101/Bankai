use std::io::{self, Write, Read};
use std::net::TcpStream;

pub fn connect_reverse_shell() -> Result<(), io::Error> {
    // Prompt for IP address
    println!("Enter the IP address of the remote server:");
    let mut ip = String::new();
    io::stdin()
        .read_line(&mut ip)
        .expect("Failed to read IP address");

    // Prompt for port number
    println!("Enter the port number of the remote server:");
    let mut port = String::new();
    io::stdin()
        .read_line(&mut port)
        .expect("Failed to read port number");

    // Trim and parse user inputs
    let ip = ip.trim();
    let port: u16 = port.trim().parse().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid port number. Please enter a valid number.")
    })?;

    // Combine IP and port into an address
    let addr = format!("{}:{}", ip, port);

    // Attempt to connect to the server
    let mut stream = TcpStream::connect(&addr).map_err(|e| {
        eprintln!("Failed to connect to {}: {}", addr, e);
        e
    })?;

    println!("Connected to {}!", addr);

    // Main communication loop
    loop {
        // Prompt for command
        println!("Enter command:");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let command = command.trim();
        if command.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        // Send command to the server
        stream.write_all(command.as_bytes()).map_err(|e| {
            eprintln!("Failed to send command: {}", e);
            e
        })?;

        // Send a newline to ensure the server knows the command is complete
        stream.write_all(b"\n").map_err(|e| {
            eprintln!("Failed to send newline: {}", e);
            e
        })?;

        // Wait for response from the server
        let mut response = vec![0; 1024]; // Buffer to hold the response
        match stream.read(&mut response) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Server closed the connection.");
                    break;
                }
                let response_str = String::from_utf8_lossy(&response[..bytes_read]);
                println!("Server response: {}", response_str);
            }
            Err(e) => {
                eprintln!("Failed to read response: {}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}
