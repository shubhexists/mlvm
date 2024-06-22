use std::path::PathBuf;

use crate::node::utils::utils::create_symbolic_link;
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

    println!("{}", version_dir_path.display());

    create_symbolic_link(&version_dir_path, version)
}
