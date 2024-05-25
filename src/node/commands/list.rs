use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

pub fn list() {
    let versions_dir_path: PathBuf = dirs::home_dir().unwrap().join(".mvm/node").join("versions");
    let installed_versions: Vec<String> = fs::read_dir(&versions_dir_path)
        .unwrap()
        .map(|file: Result<DirEntry, io::Error>| file.unwrap().path())
        .map(|file: PathBuf| file.file_name().unwrap().to_str().unwrap().to_string())
        .collect();
    if installed_versions.is_empty() {
        println!("No versions installed");
    } else {
        println!("Installed versions:");
        for version in installed_versions {
            println!("{}", version);
        }
    }
}
