use std::collections::HashMap;

use regex::Regex;

use crate::config::Config;

/// Verifies that the GitHub user with a given username belongs to a given team
/// within a given org.
pub(crate) fn check_allowed(username: &str, config: Config) -> bool {
    // TODO: performance: (optionally) cache network requests
    match (config.team, config.org, config.token) {
        (Some(team), Some(org), Some(token)) => {
            check_allowed_network(username, &*team, &*org, &*token).unwrap()
        }
        _ => false,
    }
}

/// Makes a network roundtrip to the GitHub HTTP API to accomplish
/// check_allowed.
fn check_allowed_network(
    username: &str,
    team: &str,
    org: &str,
    token: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(format!(
            "https://api.github.com/orgs/{}/teams/{}/memberships/{}",
            org, team, username
        ))
        .header("Authorization", format!("token {}", token))
        .header(
            "User-Agent",
            format!("aeneid / {}", env!("CARGO_PKG_VERSION")),
        )
        .send()?
        .json::<HashMap<String, String>>()?;

    match resp.get("state") {
        None => Ok(false),
        Some(state) => Ok(state.eq("active")),
    }
}

/// Validates a unix username, and if valid, returns the corresponding GitHub
/// username.
///
/// People with numbers in their GitHub username must login with
/// _githubUsername. Everyone else can just login with githubUsername.
///
/// This is required for the following reasons...
/// 1. Unix usernames may not start with a number, but GitHub usernames may.
/// 2. GitHub usernames may not contain an underscore, but Unix usernames may.
pub fn unix_to_github(unix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prefixed = Regex::new(r"^_[a-zA-Z0-9]*$").unwrap();
    let normal = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();

    assert!(unix.is_ascii());
    if unix.len() > 0 {
        if prefixed.is_match(&*unix) {
            return Ok(unix[1..].to_string().to_ascii_lowercase());
        }
        if normal.is_match(&*unix) {
            return Ok(unix.to_ascii_lowercase());
        }
    }

    Err("invalid username".into())
}
