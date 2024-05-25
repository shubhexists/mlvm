use std::path::PathBuf;
use std::process::{Command, Output};

pub fn exec(version: &str, file_path: &str) {
    println!("{file_path}");
    let version_dir_path: PathBuf = dirs::home_dir()
        .unwrap()
        .join(".mvm/node")
        .join("versions")
        .join(format!("v{}", version))
        .join("bin");

    if !version_dir_path.exists() {
        println!("Version {} not found", version);
        return;
    }

    let command_output: Output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source ./scripts/node.sh && remove_mvm_from_path && add_mvm_to_path {} && node -v && npm -v",
            version_dir_path.to_str().unwrap()
        ))
        .output()
        .expect("Failed to execute command");

    if command_output.status.success() {
        // Convert the output bytes to a UTF-8 string and print it
        let output_str = String::from_utf8_lossy(&command_output.stdout);
        println!("Output: {}", output_str);
    } else {
        // Print the error message captured from stderr
        let error_str = String::from_utf8_lossy(&command_output.stderr);
        eprintln!("Error: {}", error_str);
    }
    println!("Version {} is now in use", version);
}
