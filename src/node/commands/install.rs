use crate::{
    node::{
        utils::{
            unix_utils,
            utils::{
                check_already_installed, check_path_variable, create_symbolic_link,
                get_concrete_install_version,
            },
            windows_utils,
        },
        BASE_URL,
    },
    utils::{download_file, extract_file},
};
use os_info::Info;
use std::{
    error::Error,
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

pub fn install(version: &str, debug: bool, nodefault: bool) {
    if debug {
        println!("Debug mode: ON");
        println!("Installing version: {}", version);
        println!("No default flag: {}", nodefault);
    }

    let os_info: Info = os_info::get();
    if debug {
        println!("Detected OS: {:?}", os_info.os_type());
    }

    let version: &str = version.trim_start_matches("v");
    let version: Result<String, Box<dyn Error>> = get_concrete_install_version(version.to_string());
    let version: String = match version {
        Ok(version) => {
            if debug {
                println!("Concrete version to install: {}", version);
            }
            version
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match check_already_installed(&version) {
        Ok(_) => {
            println!("Version {} already installed", version);
            return;
        }
        Err(_) => {
            if debug {
                println!(
                    "Version {} not installed, proceeding with installation",
                    version
                );
            }
        }
    }

    let download_url: String = match os_info.os_type() {
        os_info::Type::Windows => windows_utils::create_windows_download_link(BASE_URL, &version),
        _ => unix_utils::create_linux_download_link(BASE_URL, &version),
    };
    println!("Downloading from: {}", download_url);

    let cache_dir: PathBuf = dirs::home_dir()
        .expect("Cannot get cache directory")
        .join(".mvm/node/.cache/bin");
    if debug {
        println!("Cache directory: {:?}", cache_dir);
    }

    let file_name: &str = download_url.split('/').last().unwrap();
    let split_file: Vec<&str> = file_name.split('.').collect();
    let temp_dir_name: String = format!("{}.{}.{}", split_file[0], split_file[1], split_file[2]);
    let temp_dir: PathBuf = cache_dir.join(&temp_dir_name);
    if debug {
        println!("Creating temporary directory: {:?}", temp_dir);
    }
    fs::create_dir_all(&temp_dir).expect("Cannot create temp directory");

    let file_: PathBuf = temp_dir.join(file_name);
    if debug {
        println!("Downloading to file: {:?}", file_);
    }
    let download: Result<(), Box<dyn Error>> = download_file(&download_url, &file_);
    match download {
        Ok(_) => {
            println!("\nDownloaded to: {:?}", cache_dir);
            let extract_path: PathBuf = temp_dir.join("files");
            if debug {
                println!("Creating extract directory: {:?}", extract_path);
            }
            fs::create_dir_all(&extract_path).expect("Cannot create files directory");

            if debug {
                println!("Extracting file: {:?}", file_);
            }
            let extract: Result<(), Box<dyn Error>> =
                extract_file(&file_.to_str().unwrap(), &extract_path.to_str().unwrap());

            let files_dir: PathBuf = extract_path.join(&temp_dir_name);
            let files: Vec<PathBuf> = fs::read_dir(&files_dir)
                .expect("Cannot read files directory")
                .map(|file: Result<DirEntry, io::Error>| file.unwrap().path())
                .collect();

            if debug {
                println!("Moving extracted files to: {:?}", extract_path);
            }
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
                    if debug {
                        println!("Creating version directory: {:?}", version_dir);
                    }
                    fs::create_dir_all(&version_dir).expect("Cannot create version directory");

                    let files: Vec<PathBuf> = fs::read_dir(&extract_path)
                        .expect("Cannot read files directory")
                        .map(|file: Result<DirEntry, io::Error>| file.unwrap().path())
                        .collect();

                    if debug {
                        println!("Moving files to version directory");
                    }
                    for file in files {
                        let file_name: &str = file.file_name().unwrap().to_str().unwrap();
                        let new_file: PathBuf = version_dir.join(file_name);
                        fs::rename(&file, &new_file).expect("Cannot move file");
                    }

                    if debug {
                        println!("Removing temporary directory: {:?}", temp_dir);
                    }
                    fs::remove_dir_all(&temp_dir).expect("Cannot remove extract directory");

                    if !nodefault {
                        if debug {
                            println!("Creating symbolic link for version {}", version);
                        }
                        create_symbolic_link(&version_dir, &version);
                        let current_file_path: PathBuf = dirs::home_dir()
                            .unwrap()
                            .join(".mvm")
                            .join("node")
                            .join("aliases")
                            .join("default");
                        if debug {
                            println!("Setting default version to: {}", version);
                        }
                        fs::write(current_file_path, version.clone()).unwrap();
                        println!("Node Version {} now in use", version);
                        check_path_variable()
                    } else if debug {
                        println!("Skipping default version setting (nodefault flag is set)");
                    }
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

    if debug {
        println!("Installation completed successfully");
    }
}
