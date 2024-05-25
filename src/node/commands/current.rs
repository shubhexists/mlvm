use std::{fs, path::PathBuf};

pub fn current() {
    let aliases_dir: PathBuf = dirs::home_dir().unwrap().join(".nvm/node/alias");
    let default_file: PathBuf = aliases_dir.join("default");
    let default_version: String = match fs::read_to_string(&default_file) {
        Ok(version) => version,
        Err(_) => {
            println!("No default version set");
            return;
        }
    };
    println!("Default version: {}", default_version);
}
