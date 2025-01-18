use std::net::TcpStream;
use std::io::{self, Write};
use std::time::Duration;

pub fn port_scanner() {
    println!("Port Scanner");

    // Prompt for target IP address
    print!("Enter the target IP address (e.g., 192.168.1.1): ");
    io::stdout().flush().unwrap();
    let mut target_ip = String::new();
    io::stdin().read_line(&mut target_ip).unwrap();
    let target_ip = target_ip.trim();

    // Prompt for port range
    print!("Enter the starting port (e.g., 1): ");
    io::stdout().flush().unwrap();
    let mut start_port = String::new();
    io::stdin().read_line(&mut start_port).unwrap();
    let start_port: u16 = match start_port.trim().parse() {
        Ok(port) if port > 0 => port,
        _ => {
            eprintln!("Invalid starting port. Please enter a valid port number.");
            return;
        }
    };

    print!("Enter the ending port (e.g., 65535): ");
    io::stdout().flush().unwrap();
    let mut end_port = String::new();
    io::stdin().read_line(&mut end_port).unwrap();
    let end_port: u16 = match end_port.trim().parse() {
        Ok(port) if port >= start_port && port <= 65535 => port,
        _ => {
            eprintln!("Invalid ending port. Please enter a valid port number.");
            return;
        }
    };

    println!(
        "[+] Scanning ports on {} from {} to {}...",
        target_ip, start_port, end_port
    );

    // Scan ports in the specified range
    for port in start_port..=end_port {
        match TcpStream::connect_timeout(
            &format!("{}:{}", target_ip, port).parse().unwrap(),
            Duration::from_secs(1),
        ) {
            Ok(_) => {
                println!("[+] Port {} is open.", port);
            }
            Err(_) => {
                // Port is closed or filtered
            }
        }
    }

    println!("[+] Port scanning complete.");
}
