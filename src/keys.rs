/// Returns a given GitHub user's stored SSH keys as a String.
pub(crate) fn get_keys(username: &str) -> String {
    get_keys_network(username).unwrap()
}

/// Makes a network roundtrip to the GitHub HTTP API to accomplish get_keys.
fn get_keys_network(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(format!("https://github.com/{}.keys", username))?.text()?;
    Ok(resp)
}
