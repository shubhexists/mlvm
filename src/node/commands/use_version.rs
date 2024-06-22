use std::path::PathBuf;

use crate::node::utils::utils::{create_symbolic_link, get_concrete_use_version};
pub fn use_version(version: &str) {
    let version_dir_path: PathBuf = dirs::home_dir()
        .unwrap()
        .join(".mvm/node")
        .join("versions")
        .join(format!("v{}", version));

    if !version_dir_path.exists() {
        println!("Version {} not found", version);
        return;
    }

    let ver: String = get_concrete_use_version(version).unwrap();
    create_symbolic_link(&version_dir_path, &ver)
}
