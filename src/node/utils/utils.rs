use crate::node::types::LTS;
use regex::Regex;
use reqwest::blocking::Client;
use std::{
    error::Error,
    ffi::OsString,
    fs::{self, DirEntry, File, ReadDir},
    path::PathBuf,
};

use crate::node::{BASE_URL, LTS};

pub fn get_concrete_version(version: String) -> Result<String, Box<dyn Error>> {
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
    let lts_dir: PathBuf = aliases_dir.join("lts");
    fs::create_dir_all(&lts_dir)?;

    let default_file: PathBuf = aliases_dir.join("default");
    File::create(&default_file)?;

    let argon_file: PathBuf = lts_dir.join("argon");
    let _write = fs::write(argon_file, "v4.9.1")?;

    let boron_file: PathBuf = lts_dir.join("boron");
    let _write = fs::write(boron_file, "v6.17.1")?;

    let carbon_file: PathBuf = lts_dir.join("carbon");
    let _write = fs::write(carbon_file, "v8.17.0")?;

    let dubnium_file: PathBuf = lts_dir.join("dubnium");
    let _write = fs::write(dubnium_file, "v10.24.1")?;

    let erbium_file: PathBuf = lts_dir.join("erbium");
    let _write = fs::write(erbium_file, "v12.22.12")?;

    let fermium_file: PathBuf = lts_dir.join("fermium");
    let _write = fs::write(fermium_file, "v14.21.3")?;

    let gallium_file: PathBuf = lts_dir.join("gallium");
    let _write = fs::write(gallium_file, "v16.20.2")?;

    let hydrogen_file: PathBuf = lts_dir.join("hydrogen");
    let _write = fs::write(hydrogen_file, "v18.20.3")?;

    let iron_file: PathBuf = lts_dir.join("iron");
    let _write = fs::write(iron_file, "v20.13.1")?;

    let versions_dir: PathBuf = node_dir.join("versions");
    fs::create_dir_all(&versions_dir)?;

    let cache_bin_dir: PathBuf = node_dir.join(".cache/bin");
    fs::create_dir_all(&cache_bin_dir)?;

    Ok(())
}

pub fn get_selection_array() -> Vec<LTS> {
    let home_dir: PathBuf = dirs::home_dir().expect("Cannot get home directory");
    let mvm_dir: PathBuf = home_dir.join(".mvm");
    let node_dir: PathBuf = mvm_dir.join("node");
    let aliases_dir: PathBuf = node_dir.join("aliases");
    let lts_dir: PathBuf = aliases_dir.join("lts");
    let mut selection_array: Vec<LTS> = Vec::new();
    selection_array.push(LTS {
        version: LTS.to_string(),
        alias: "lts".to_string(),
    });
    let files: ReadDir = fs::read_dir(lts_dir).unwrap();
    for file in files {
        let file: DirEntry = file.unwrap();
        let file_name: OsString = file.file_name();
        let file_name: &str = file_name.to_str().unwrap();
        if file_name == "lts" {
            continue;
        }
        let file_path: PathBuf = file.path();
        let content: String = fs::read_to_string(file_path).unwrap();
        selection_array.push(LTS {
            version: content,
            alias: file_name.to_string(),
        });
    }
    selection_array
}
