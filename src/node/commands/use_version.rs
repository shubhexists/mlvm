use crate::node::utils::utils::{
    check_path_variable, create_symbolic_link, get_concrete_use_version,
};
use std::{fs, path::PathBuf};

pub fn use_version(version: &str, debug: bool) {
    let ver: String = match get_concrete_use_version(version) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Error getting the concrete use version: {}", err);
            return;
        }
    };

    if debug {
        println!("Using version: {}", ver);
    }

    let version_dir_path: PathBuf = dirs::home_dir()
        .unwrap()
        .join(".mvm/node")
        .join("versions")
        .join(format!("v{}", ver));

    if debug {
        println!(
            "Creating symbolic link for version directory: {:?}",
            version_dir_path
        );
    }
    create_symbolic_link(&version_dir_path, &ver);

    let current_file_path: PathBuf = dirs::home_dir()
        .unwrap()
        .join(".mvm/node")
        .join("aliases")
        .join("default");

    if debug {
        println!("Writing current version to file: {:?}", current_file_path);
    }
    fs::write(&current_file_path, ver.clone()).unwrap();

    println!("Node Version {} now in use", ver);
    check_path_variable();

    if debug {
        println!("Checked PATH variable after switching versions.");
    }
}
