use clap::{Parser, Subcommand};
use std::io::{self, Write}; // For input and output
mod overflow;
mod reverseShellCon;
use std::process::Command;

#[derive(Parser)]
#[command(name = "vulnerability_tester")]
#[command(author = "Diogo Andrade & Tiago Pereira")]
#[command(version = "1.0")]
#[command(about = "A CLI tool to test various vulnerabilities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Test for buffer overflow vulnerability
    BufferOverflow {},
    
    /// Simulate a reverse shell attack
    ReverseShell {},
    
    /// Test for ransomware behavior
    Ransomware {},
    
    /// Simulate a rootkit attack
    Rootkit {},
}

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        loop {
            clear_terminal();
            println!(" ____                    _              _ ");
            println!("| __ )    __ _   _ __   | | __   __ _  (_)");
            println!("|  _ \\   / _` | | '_ \\  | |/ /  / _` | | |");
            println!("| |_) | | (_| | | | | | |   <  | (_| | | |");
            println!("|____/   \\__,_| |_| |_| |_\\_\\  \\__,_ | |_|  ");
            println!();

            // Print the options in two columns
            println!("1. Buffer Overflow        2. Reverse Shell");
            println!("3. Ransomware             4. Rootkit");
            println!("5. Exit");
            println!();

            print!("Enter the number of your choice: ");
            io::stdout().flush().unwrap(); // Ensure the prompt shows immediately

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => test_buffer_overflow(),
                "2" => simulate_reverse_shell(),
                "3" => test_ransomware(),
                "4" => simulate_rootkit(),
                "5" => {
                    println!("Exiting...");
                    break;
                }
                _ => println!("Invalid choice. Please try again."),
            }
        }
    } else {
        // Handle single command directly if passed from the command line (non-interactive mode)
        match cli.command.unwrap() {
            Commands::BufferOverflow {} => test_buffer_overflow(),
            Commands::ReverseShell {} => simulate_reverse_shell(),
            Commands::Ransomware {} => test_ransomware(),
            Commands::Rootkit {} => simulate_rootkit(),
        }
    }
}

fn test_buffer_overflow() {
    println!("Testing buffer overflow vulnerability...");
    overflow::test(); // Call the function from the overflow module

}

fn simulate_reverse_shell() {
    println!("Simulating reverse shell attack...");
    match reverseShellCon::start_reverse_shell_server() {
        Ok(_) => println!("Reverse shell session ended."),
        Err(e) => eprintln!("Error: {}", e),
    }
}


fn test_ransomware() {
    println!("Testing ransomware behavior...");
    // Add the logic for ransomware test here
}

fn simulate_rootkit() {
    println!("Simulating rootkit attack...");
    // Add the logic for rootkit simulation here
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
