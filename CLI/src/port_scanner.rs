use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

pub fn port_scanner() {
    println!("Port Scanner");

    // Prompt for target IP address
    print!("Enter the target IP address (e.g., 192.168.1.1): ");
    io::stdout().flush().unwrap();
    let mut target_ip = String::new();
    io::stdin().read_line(&mut target_ip).unwrap();
    let target_ip = target_ip.trim().to_string();

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
	    Ok(port) if port >= start_port => port,
	    _ => {
		eprintln!("Invalid ending port. Please enter a valid port number.");
		return;
	    }
	};


    println!(
        "[+] Scanning ports on {} from {} to {}...",
        target_ip, start_port, end_port
    );

    // Port-to-service mapping
    let service_map = Arc::new(get_service_map());

    // Results storage
    let results = Arc::new(Mutex::new(Vec::new()));

    // Create threads for parallel scanning
    let threads: Vec<_> = (start_port..=end_port)
        .map(|port| {
            let target_ip = target_ip.clone();
            let service_map = Arc::clone(&service_map);
            let results = Arc::clone(&results);

            thread::spawn(move || {
                let socket_addr = format!("{}:{}", target_ip, port);
                match TcpStream::connect_timeout(
                    &socket_addr.parse::<SocketAddr>().unwrap(),
                    Duration::from_secs(1),
                ) {
                    Ok(_) => {
                        let service = service_map.get(&port).cloned().unwrap_or("Unknown".to_string());
                        let mut results = results.lock().unwrap();
                        results.push((port, service));
                    }
                    Err(_) => {
                        // Port is closed or filtered
                    }
                }
            })
        })
        .collect();

    // Wait for all threads to complete
    for thread in threads {
        thread.join().unwrap();
    }

    // Print results
    let results = results.lock().unwrap();
    if results.is_empty() {
        println!("[+] No open ports found.");
    } else {
        println!("[+] Open ports:");
        for (port, service) in results.iter() {
            println!("Port {}: {}", port, service);
        }
    }

    println!("[+] Port scanning complete.");
}

// Common port-to-service mapping
fn get_service_map() -> HashMap<u16, String> {
    let mut map = HashMap::new();
    map.insert(22, "SSH".to_string());
    map.insert(80, "HTTP".to_string());
    map.insert(443, "HTTPS".to_string());
    map.insert(21, "FTP".to_string());
    map.insert(25, "SMTP".to_string());
    map.insert(110, "POP3".to_string());
    map.insert(143, "IMAP".to_string());
    map.insert(53, "DNS".to_string());
    map.insert(3306, "MySQL".to_string());
    map.insert(5432, "PostgreSQL".to_string());
    map
}
