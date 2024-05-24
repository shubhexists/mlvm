pub fn create_windows_download_link(base_url: &str, version: &str) -> String {
    let download_url: String =
        base_url.to_string() + "v" + version + "/node-" + version + "-win-x64.zip";
    download_url
}
