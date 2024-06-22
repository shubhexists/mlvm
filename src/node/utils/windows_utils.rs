use std::path;
use std::path::PathBuf;
use symlink::remove_symlink_dir;
use symlink::symlink_dir;

pub fn create_windows_download_link(base_url: &str, version: &str) -> String {
    let download_url: String =
        base_url.to_string() + "v" + version + "/node-" + version + "-win-x64.zip";
    download_url
}

pub fn create_windows_symbolic_link(version_dir: &PathBuf, version: &str) {
    let bin_dir: path::PathBuf = version_dir.join("bin");
    let link_path: path::PathBuf = dirs::home_dir().unwrap().join(".mvm/node/bin");

    if !bin_dir.exists() {
        panic!("Node version {} not found", version);
    }

    if !bin_dir.exists() {
        panic!("Node version {} not found", version);
    }

    if link_path.exists() {
        remove_symlink_dir(&link_path).unwrap();
    }

    symlink_dir(&bin_dir, &link_path).unwrap();
    println!("Symlink created successfully !")
}