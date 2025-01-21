use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::process::{Command};

mod overflow;
mod reverse_shell;
mod dll_hijacking_c;
mod phishing;
mod decrypt;
mod port_scanner;
mod keylogger;

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
        keylogger::handle_background_keylogger();
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
            println!("7. Decrypt                8. Exit");
            println!();

            print!("Enter the number of your choice: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => test_buffer_overflow(),
                "2" => reverse_shell::simulate_reverse_shell(),
                "3" => port_scanner::port_scanner(),
                "4" => match keylogger::deploy_keylogger_detached() {
		    Ok(_) => println!("Keylogger is running in the background."),
		    Err(e) => eprintln!("Failed to start keylogger: {}", e),
		},
                "5" => match dll_hijacking_c::compile_c_to_dll() {
                    Ok(_) => println!("DLL compiled successfully."),
                    Err(e) => eprintln!("Compilation error: {}", e),
                },
                "6" => phishing::generate_email(),
                "7" => decrypt::decrypt_documents_cli(),
                "8" => {
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
	    Commands::Keylogger {} => {
		if let Err(e) = keylogger::deploy_keylogger_detached() {
		    eprintln!("Failed to start keylogger: {}", e);
		}
	    },
	    Commands::DllHijacking {} => {
		if let Err(e) = dll_hijacking_c::compile_c_to_dll() {
		    eprintln!("Compilation error: {}", e);
		}
	    },
	    Commands::DecryptDocuments {} => decrypt::decrypt_documents_cli(),
	}
    }
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
