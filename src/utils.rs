use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use tar::Archive;
use xz2::read::XzDecoder;

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

fn extract_tar_gz(archive_path: &str, extract_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive_path)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(extract_path)?;
    Ok(())
}

fn extract_tar_xz(archive_path: &str, extract_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive_path)?;
    let tar = XzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(extract_path)?;
    Ok(())
}

pub fn extract_file(archive_path: &str, extract_path: &str) -> Result<(), Box<dyn Error>> {
    if archive_path.ends_with(".tar.gz") {
        extract_tar_gz(archive_path, extract_path)?;
    } else if archive_path.ends_with(".tar.xz") {
        extract_tar_xz(archive_path, extract_path)?;
    }
    Ok(())
}
