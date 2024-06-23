use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

pub fn remove(version: &str, debug: bool) {
    let versions_dir_path: PathBuf = dirs::home_dir().unwrap().join(".mvm/node").join("versions");
    let installed_versions: Vec<String> = fs::read_dir(&versions_dir_path)
        .unwrap()
        .map(|file: Result<DirEntry, io::Error>| file.unwrap().path())
        .map(|file: PathBuf| file.file_name().unwrap().to_str().unwrap().to_string())
        .collect();
    if installed_versions.contains(&format!("v{}", version)) {
        let version_dir_path: PathBuf = versions_dir_path.join(format!("v{}", version));
        fs::remove_dir_all(&version_dir_path).unwrap();
        println!("Version {} removed Successfully !", version);
    } else {
        println!("Version not found");
    }
}
