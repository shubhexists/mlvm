use std::{fs, path::PathBuf};

pub fn current(debug: bool) {
    let aliases_dir: PathBuf = dirs::home_dir()
        .unwrap()
        .join(".mvm")
        .join("node")
        .join("aliases");
    let default_file: PathBuf = aliases_dir.join("default");

    if !default_file.exists() {
        println!("No default version set");
        return;
    }

    if debug {
        println!("Reading default version from: {:?}", default_file);
    }

    let default_version: String = match fs::read_to_string(&default_file) {
        Ok(version) => version,
        Err(err) => {
            println!("Error reading default file: {}", err);
            return;
        }
    };

    if debug {
        println!("Read default version: {}", default_version);
    }

    println!("Default version: {}", default_version);
}
