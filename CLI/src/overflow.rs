use std::net::TcpStream;
use std::io::{Read, Write}; // Necessary imports for TcpStream operations

pub fn test() {
    println!("Testing buffer overflow vulnerability...");

    // Connect to the server
    let server_addr = "127.0.0.1:8080".parse::<std::net::SocketAddr>().unwrap();
    let mut stream = TcpStream::connect(server_addr).expect("Failed to connect to server");

    // Send a crafted request to overflow a buffer
    let overflow_payload = "A".repeat(100); // Crafted payload to overflow a buffer
    let _request = format!("UPLOAD /tmp/overflow.txt {}\n", overflow_payload);
    stream.write_all(_request.as_bytes()).expect("Failed to send request");

    // Read the response from the server
    let mut response = String::new();
    let mut buffer = [0; 1024];
    while let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            break;
        }
        response.push_str(std::str::from_utf8(&buffer[..n]).unwrap());
    }

    // Print the server response
    println!("Response: {}", response);
}
