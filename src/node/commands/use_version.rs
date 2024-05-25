use std::path::PathBuf;
pub fn use_version(version: &str) {
    let version_dir_path: PathBuf = dirs::home_dir()
        .unwrap()
        .join(".mvm/node")
        .join("versions")
        .join(format!("v{}", version))
        .join("bin");

    if !version_dir_path.exists() {
        println!("Version {} not found", version);
        return;
    }
}
