use std::{
    error::Error,
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

use crate::{
    node::{
        utils::{unix_utils, utils::get_concrete_version, windows_utils},
        BASE_URL,
    },
    utils::{download_file, extract_file},
};

pub fn install(version: &str) {
    let os_info = os_info::get();
    let version: &str = version.trim_start_matches("v");
    let version: Result<String, Box<dyn Error>> = get_concrete_version(version.to_string());
    let version: String = match version {
        Ok(version) => version,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let download_url: String = match os_info.os_type() {
        os_info::Type::Windows => windows_utils::create_windows_download_link(BASE_URL, &version),
        _ => unix_utils::create_linux_download_link(BASE_URL, &version),
    };
    println!("Downloading from: {}", download_url);
    let cache_dir: PathBuf = dirs::home_dir()
        .expect("Cannot get cache directory")
        .join(".mvm/node/.cache/bin");
    let file_name: &str = download_url.split('/').last().unwrap();
    let split_file: Vec<&str> = file_name.split('.').collect();
    let temp_dir_name: String = format!("{}.{}.{}", split_file[0], split_file[1], split_file[2]);
    let temp_dir: PathBuf = cache_dir.join(&temp_dir_name);
    fs::create_dir_all(&temp_dir).expect("Cannot create temp directory");
    let file_: PathBuf = temp_dir.join(file_name);
    let download: Result<(), Box<dyn Error>> = download_file(&download_url, &file_);
    match download {
        Ok(_) => {
            println!("Downloaded to: {:?}", cache_dir);
            let extract_path: PathBuf = temp_dir.join("files");
            fs::create_dir_all(&extract_path).expect("Cannot create files directory");
            let extract: Result<(), Box<dyn Error>> =
                extract_file(&file_.to_str().unwrap(), &extract_path.to_str().unwrap());
            let files_dir: PathBuf = extract_path.join(&temp_dir_name);
            let files: Vec<PathBuf> = fs::read_dir(&files_dir)
                .expect("Cannot read files directory")
                .map(|file: Result<DirEntry, io::Error>| file.unwrap().path())
                .collect();
            for file in files {
                let file_name: &str = file.file_name().unwrap().to_str().unwrap();
                let new_file: PathBuf = extract_path.join(file_name);
                fs::rename(&file, &new_file).expect("Cannot move file");
            }
            fs::remove_dir_all(&files_dir).expect("Cannot remove files directory");
            match extract {
                Ok(_) => {
                    println!("Extracted to: {:?}", extract_path);
                    let versions_dir: PathBuf = dirs::home_dir()
                        .expect("Cannot get home directory")
                        .join(".mvm/node/versions");
                    let version_dir: PathBuf = versions_dir.join(format!("v{}", version));
                    fs::create_dir_all(&version_dir).expect("Cannot create version directory");
                    let files: Vec<PathBuf> = fs::read_dir(&extract_path)
                        .expect("Cannot read files directory")
                        .map(|file: Result<DirEntry, io::Error>| file.unwrap().path())
                        .collect();
                    for file in files {
                        let file_name: &str = file.file_name().unwrap().to_str().unwrap();
                        let new_file: PathBuf = version_dir.join(file_name);
                        fs::rename(&file, &new_file).expect("Cannot move file");
                    }
                    fs::remove_dir_all(&temp_dir).expect("Cannot remove extract directory");
                }
                Err(e) => {
                    println!("{}", e);
                    panic!("Cannot extract file");
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            panic!("Cannot download file");
        }
    }
}
