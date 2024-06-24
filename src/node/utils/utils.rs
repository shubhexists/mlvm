use super::{unix_utils, windows_utils};
use crate::node::BASE_URL;
use os_info::Info;
use regex::Regex;
use reqwest::blocking::Client;
use std::{
    env,
    error::Error,
    fs::{self, DirEntry, File},
    io,
    path::PathBuf,
};

pub fn get_concrete_install_version(version: String) -> Result<String, Box<dyn Error>> {
    let version_parts: Vec<&str> = version.split('.').collect();
    if version_parts.len() == 1 {
        let all_versions: Vec<String> = get_available_node_versions(Some(&version)).unwrap();
        if all_versions.len() > 0 {
            let mut versions: Vec<i32> = Vec::new();
            let largest_minor_version: i32 = all_versions
                .iter()
                .map(|ver: &String| {
                    let ver_parts: Vec<&str> = ver.split('.').collect();
                    ver_parts[1].parse::<i32>().unwrap()
                })
                .max()
                .unwrap();
            for ver in all_versions {
                let ver_parts: Vec<&str> = ver.split('.').collect();
                if ver_parts[1].parse::<i32>().unwrap() == largest_minor_version {
                    versions.push(ver_parts[2].parse::<i32>().unwrap());
                }
            }
            let largest_patch_version: &i32 = versions.iter().max().unwrap();
            let final_version: String = format!(
                "{}.{}.{}",
                version, largest_minor_version, largest_patch_version
            );
            return Ok(final_version);
        } else {
            return Err("Version not found".into());
        }
    } else if version_parts.len() == 2 {
        let all_versions: Vec<String> = get_available_node_versions(Some(&version)).unwrap();
        if all_versions.len() > 1 {
            let mut versions: Vec<i32> = Vec::new();
            for ver in all_versions {
                let ver_parts: Vec<&str> = ver.split('.').collect();
                versions.push(ver_parts[2].parse::<i32>().unwrap());
            }
            let largest_patch_version: &i32 = versions.iter().max().unwrap();
            let final_version: String = format!("{}.{}", version, largest_patch_version);
            return Ok(final_version);
        } else if all_versions.len() == 1 {
            return Ok(all_versions[0].clone());
        } else {
            return Err("Version not found".into());
        }
    } else if version_parts.len() == 3 {
        let all_versions: Vec<String> = get_available_node_versions(None).unwrap();
        if all_versions.contains(&version) {
            return Ok(version);
        } else {
            return Err("Version not found".into());
        }
    } else {
        return Err("Invalid version".into());
    }
}

fn get_available_node_versions(version: Option<&String>) -> Result<Vec<String>, Box<dyn Error>> {
    let url: &str = BASE_URL;
    let client: Client = Client::new();
    let response: reqwest::blocking::Response = client.get(url).send()?;
    let body: String = response.text()?;
    let re: Regex = Regex::new(r#"href="v(\d+\.\d+\.\d+)/""#)?;
    let mut versions: Vec<String> = Vec::new();

    for cap in re.captures_iter(&body) {
        let ver: String = cap[1].to_string();
        match &version {
            Some(version) => {
                let ver_parts: Vec<&str> = ver.split('.').collect();
                let version_parts: Vec<&str> = version.split('.').collect();

                if version_parts.len() <= ver_parts.len()
                    && version_parts.iter().zip(&ver_parts).all(|(a, b)| a == b)
                {
                    versions.push(ver);
                }
            }
            None => {
                versions.push(ver);
            }
        }
    }
    Ok(versions)
}

pub fn create_node_directory() -> Result<(), Box<dyn Error>> {
    let home_dir: PathBuf = dirs::home_dir().expect("Cannot get home directory");
    let mvm_dir: PathBuf = home_dir.join(".mvm");
    let node_dir: PathBuf = mvm_dir.join("node");
    let aliases_dir: PathBuf = node_dir.join("aliases");

    fs::create_dir_all(&aliases_dir)?;
    let default_file: PathBuf = aliases_dir.join("default");
    File::create(&default_file)?;

    let versions_dir: PathBuf = node_dir.join("versions");
    fs::create_dir_all(&versions_dir)?;

    let cache_bin_dir: PathBuf = node_dir.join(".cache/bin");
    fs::create_dir_all(&cache_bin_dir)?;

    Ok(())
}

pub fn create_symbolic_link(version_dir: &PathBuf, version: &str) {
    let os_info: Info = os_info::get();
    match os_info.os_type() {
        os_info::Type::Windows => {
            windows_utils::create_windows_symbolic_link(version_dir, version);
        }
        _ => {
            unix_utils::create_linux_symbolic_link(version_dir, version);
        }
    }
}

pub fn get_concrete_use_version(version: &str) -> Result<String, Box<dyn Error>> {
    let version_parts: Vec<&str> = version.split('.').collect();
    let versions_dir_path: PathBuf = dirs::home_dir().unwrap().join(".mvm/node").join("versions");
    let installed_versions: Vec<String> = fs::read_dir(&versions_dir_path)
        .unwrap()
        .map(|file: Result<DirEntry, io::Error>| file.unwrap().path())
        .map(|file: PathBuf| file.file_name().unwrap().to_str().unwrap().to_string())
        .map(|file: String| file.trim_start_matches("v").to_string())
        .collect();
    let splitted: Vec<Vec<&str>> = installed_versions
        .iter()
        .map(|ver: &String| ver.split('.').collect())
        .collect();
    if version_parts.len() == 1 {
        let filtered_versions: Vec<Vec<&str>> = splitted
            .iter()
            .filter(|vec: &&Vec<&str>| vec[0] == version_parts[0])
            .cloned()
            .collect();
        if filtered_versions.len() > 0 {
            let mut versions: Vec<i32> = Vec::new();
            let largest_minor_version: i32 = filtered_versions
                .iter()
                .map(|ver: &Vec<&str>| ver[1].parse::<i32>().unwrap())
                .max()
                .unwrap();
            for ver in filtered_versions {
                if ver[1].parse::<i32>().unwrap() == largest_minor_version {
                    versions.push(ver[2].parse::<i32>().unwrap());
                }
            }
            let largest_patch_version: &i32 = versions.iter().max().unwrap();
            let final_version: String = format!(
                "{}.{}.{}",
                version_parts[0], largest_minor_version, largest_patch_version
            );
            return Ok(final_version);
        } else {
            return Err(
                "No Node version is available. Try installing from `mvm node install version` "
                    .into(),
            );
        }
    } else if version_parts.len() == 2 {
        let filtered_versions: Vec<Vec<&str>> = splitted
            .iter()
            .filter(|vec: &&Vec<&str>| vec[0] == version_parts[0] && vec[1] == version_parts[1])
            .cloned()
            .collect();
        if filtered_versions.len() > 0 {
            let mut versions: Vec<i32> = Vec::new();
            for ver in filtered_versions {
                versions.push(ver[2].parse::<i32>().unwrap());
            }
            let largest_patch_version: &i32 = versions.iter().max().unwrap();
            let final_version: String = format!("{}.{}", version_parts[0], largest_patch_version);
            return Ok(final_version);
        } else {
            return Err(
                "No Node version is available. Try installing from `mvm node install version` "
                    .into(),
            );
        }
    } else if version_parts.len() == 3 {
        if installed_versions.contains(&version.to_string()) {
            return Ok(version.to_string());
        } else {
            return Err("Version not found".into());
        }
    } else {
        return Err("Invalid version".into());
    }
}

pub fn check_already_installed(version: &str) -> Result<(), Box<dyn Error>> {
    let versions_dir_path: PathBuf = dirs::home_dir().unwrap().join(".mvm/node").join("versions");
    let installed_versions: Vec<String> = fs::read_dir(&versions_dir_path)
        .unwrap()
        .map(|file: Result<DirEntry, io::Error>| file.unwrap().path())
        .map(|file: PathBuf| file.file_name().unwrap().to_str().unwrap().to_string())
        .map(|file: String| file.trim_start_matches("v").to_string())
        .collect();
    println!("version: {:?}", version);
    if installed_versions.contains(&version.to_string()) {
        return Ok(());
    } else {
        return Err("Version not installed".into());
    }
}

pub fn check_path_variable() {
    let target_path: PathBuf = dirs::home_dir().unwrap().join(".mvm/node/bin");
    match env::var("PATH") {
        Ok(paths) => {
            let paths: Vec<&str> = paths.split(':').collect();
            if !paths.contains(&target_path.to_str().unwrap()) {
                println!("PATH variable does not contain mvm node bin directory.");
                println!("Add to your bashrc to start using mvm:\n\t export PATH=/home/jerry/.mvm/node/bin:$PATH")
            }
        }
        Err(_) => {
            println!("PATH variable not found in shell configuration");
        }
    }
}
