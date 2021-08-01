use std::collections::HashMap;

use crate::Config;

/// Verifies that the GitHub user with a given username belongs to a given team
/// within a given org.
pub(crate) fn check_allowed(username: &str, config: Config) -> bool {
    if config.overrides.contains(&*username) {
        return true;
    }

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
        .header("User-Agent", "aeneid / 0.5.0")
        .send()?
        .json::<HashMap<String, String>>()?;

    match resp.get("state") {
        None => Ok(false),
        Some(state) => Ok(state.eq("active")),
    }
}
