use std::{fs, path::PathBuf};

pub fn current(debug: bool) {
    let aliases_dir: PathBuf = dirs::home_dir().unwrap().join(".mvm/node/aliases");
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
