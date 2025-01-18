use std::process::Command;

pub fn compile_c_to_exe() -> Result<(), String> {
    // Path to the C file and desired output EXE
    let c_file = "decrypt.c";
    let output_exe = "decrypt.exe";

    // Define the compiler and arguments
    let compiler = "i686-w64-mingw32-gcc"; // Cross-compiler
    let args = [
        "-o", output_exe,       // Output file
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
        println!("Compilation succeeded. EXE created: {}", output_exe);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Compilation failed: {}", stderr))
    }
}
