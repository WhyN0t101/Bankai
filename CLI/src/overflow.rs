use std::io::{Read, Write}; // Necessary imports for TcpStream operations
use std::net::{TcpStream, SocketAddr};

pub fn test() {

    let server_addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let mut stream = TcpStream::connect(server_addr).expect("Failed to connect to server");

    let overflow_payload = "A".repeat(2048);  // Increase the size to exceed buffer

    let _request = format!("UPLOAD /tmp/overflow.txt {}\n", overflow_payload);

    stream.write_all(_request.as_bytes()).expect("Failed to send request");

    let mut response = String::new();
    let mut buffer = [0; 1024]; 
    while let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            break;  
        }
        response.push_str(std::str::from_utf8(&buffer[..n]).unwrap());
    }


    println!("Response: {}", response);
}
