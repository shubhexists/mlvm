use std::{fs, path::PathBuf};

use crate::node::utils::utils::{
    check_path_variable, create_symbolic_link, get_concrete_use_version,
};
pub fn use_version(version: &str, debug: bool) {
    let ver: String = get_concrete_use_version(version).unwrap();
    let version_dir_path: PathBuf = dirs::home_dir()
        .unwrap()
        .join(".mvm/node")
        .join("versions")
        .join(format!("v{}", ver));
    create_symbolic_link(&version_dir_path, &ver);
    let current_file_path: PathBuf = dirs::home_dir()
        .unwrap()
        .join(".mvm/node")
        .join("aliases")
        .join("default");
    fs::write(current_file_path, ver.clone()).unwrap();
    println!("Node Version {} now in use", ver);
    check_path_variable();
}
