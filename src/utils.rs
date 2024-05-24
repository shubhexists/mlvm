use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use reqwest::blocking::Client;

pub fn create_mvm_directory() -> Result<(), Box<dyn Error>> {
    let home_dir: PathBuf = dirs::home_dir().expect("Cannot get home directory");
    let mvm_dir: PathBuf = home_dir.join(".mvm");
    fs::create_dir_all(&mvm_dir)?;
    Ok(())
}

pub fn download_file(url: &str, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let client: Client = Client::new();
    let response: reqwest::blocking::Response = client.get(url).send()?;
    let mut file: File = File::create(path)?;
    let body: Vec<u8> = response.bytes()?.to_vec();
    file.write_all(&body)?;
    Ok(())
}
