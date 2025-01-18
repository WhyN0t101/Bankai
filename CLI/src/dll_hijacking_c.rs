use std::process::Command;

pub fn compile_c_to_dll() -> Result<(), String> {
    // Path to the C file and desired output DLL
    let c_file = "wireshark.c";
    let output_dll = "airpcap.dll";

    // Define the compiler and arguments
    let compiler = "i686-w64-mingw32-gcc"; // Cross-compiler
    let args = [
        "-shared",              // Create a shared library (DLL)
        "-o", output_dll,       // Output file
        c_file,                 // Input C file
        "-luser32",             // Link against the user32 library
    ];

    // Execute the compiler command
    let output = Command::new(compiler)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute compiler: {}", e))?;

    // Check if the command was successful
    if output.status.success() {
        println!("Compilation succeeded. DLL created: {}", output_dll);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Compilation failed: {}", stderr))
    }
}
