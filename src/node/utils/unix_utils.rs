pub fn create_linux_download_link(base_url: &str, version: &str) -> String {
    let download_url: String =
        base_url.to_string() + "v" + version + "/node-v" + version + "-linux-x64.tar.xz";
    download_url
}
