use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

pub fn remove(version: &str, debug: bool) {
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    let versions_dir_path: PathBuf = home_dir.join(".mvm/node").join("versions");
    let aliases_dir_path: PathBuf = home_dir.join(".mvm/node/aliases");
    let default_file_path: PathBuf = aliases_dir_path.join("default");

    let current_version: String = fs::read_to_string(&default_file_path)
        .unwrap_or_else(|_| {
            if debug {
                eprintln!("No default version found. Proceeding without current version check.");
            }
            String::new()
        })
        .trim()
        .to_string();

    if current_version == format!("v{}", version) {
        eprintln!("Error: Cannot remove the current version (v{}).", version);
        return;
    }

    let installed_versions: Vec<String> = fs::read_dir(&versions_dir_path)
        .unwrap()
        .map(|file: Result<DirEntry, io::Error>| file.unwrap().path())
        .map(|file: PathBuf| file.file_name().unwrap().to_str().unwrap().to_string())
        .collect();

    if installed_versions.contains(&format!("v{}", version)) {
        let version_dir_path: PathBuf = versions_dir_path.join(format!("v{}", version));
        fs::remove_dir_all(&version_dir_path).unwrap();
        println!("Version {} removed successfully!", version);
    } else {
        println!("Version v{} not found", version);
    }
}
