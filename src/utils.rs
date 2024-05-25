use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use flate2::read::GzDecoder;
use indicatif::ProgressBar;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use std::io::Read;
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
    let mut response: reqwest::blocking::Response = client.get(url).send()?;
    let total_size = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("Response doesn't include the content length")?
        .to_str()?
        .parse::<u64>()?;
    let mut file: File = File::create(path)?;
    let pb: ProgressBar = ProgressBar::new(total_size);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );
    let mut downloaded: u64 = 0;
    let mut buffer = [0; 8192];

    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");

    Ok(())
}

fn extract_tar_gz(archive_path: &str, extract_path: &str) -> Result<(), Box<dyn Error>> {
    let file: File = File::open(archive_path)?;
    let tar: GzDecoder<File> = GzDecoder::new(file);
    let mut archive: Archive<GzDecoder<File>> = Archive::new(tar);
    archive.unpack(extract_path)?;
    Ok(())
}

fn extract_tar_xz(archive_path: &str, extract_path: &str) -> Result<(), Box<dyn Error>> {
    let file: File = File::open(archive_path)?;
    let tar: XzDecoder<File> = XzDecoder::new(file);
    let mut archive: Archive<XzDecoder<File>> = Archive::new(tar);
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
