use std::{error::Error, path::PathBuf};

use crate::{
    node::{
        utils::{unix_utils, utils::get_concrete_version, windows_utils},
        BASE_URL,
    },
    utils::download_file,
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
    match os_info.os_type() {
        os_info::Type::Windows => {
            let download_url: String =
                windows_utils::create_windows_download_link(BASE_URL, &version);
            println!("Downloading from: {}", download_url);
            let cache_dir: PathBuf = dirs::home_dir()
                .expect("Cannot get cache directory")
                .join(".mvm/node/.cache/bin");
            let file_name: &str = download_url.split('/').last().unwrap();
            let file_: PathBuf = cache_dir.join(file_name);
            let download: Result<(), Box<dyn Error>> = download_file(&download_url, &file_);
            match download {
                Ok(_) => {
                    println!("Downloaded to: {:?}", cache_dir);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        _ => {
            let download_url: String = unix_utils::create_linux_download_link(BASE_URL, &version);
            println!("Downloading from: {}", download_url);
            let cache_dir: PathBuf = dirs::home_dir()
                .expect("Cannot get cache directory")
                .join(".mvm/node/.cache/bin");
            let file_name: &str = download_url.split('/').last().unwrap();
            let file_: PathBuf = cache_dir.join(file_name);
            let download: Result<(), Box<dyn Error>> = download_file(&download_url, &file_);
            match download {
                Ok(_) => {
                    println!("Downloaded to: {:?}", cache_dir);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
