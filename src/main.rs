use std::{collections::HashSet, fs};

use clap::App;
use regex::Regex;
use serde_derive::Deserialize;

mod access;
mod keys;

/// config file in /etc/aeneid/configuration.toml
#[derive(Deserialize)]
struct Config {
    /// Version of the aeneid configuration
    cfg_version: u32,

    /// GitHub org name (https://github.com/organizations/new)
    org: Option<String>,
    /// GitHub team name (https://github.com/orgs/<org>/teams)
    team: Option<String>,
    /// GitHub access token with read:org (https://github.com/settings/tokens)
    token: Option<String>,

    /// set of GitHub users to always allow
    overrides: HashSet<String>,
}

fn main() {
    let args = App::new("aeneid")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Nikhil Jha <mail@nikhiljha.com>")
        .about("authenticate SSH users with their GitHub authorized_keys")
        .arg("<username> 'a GitHub username to query keys from'")
        .get_matches();

    // TODO: security: (footgun) validate that the config has chmod 600
    // TODO: security: (footgun) validate that AuthorizedKeysCommandRunAs is set
    let config_path =
        std::env::var("AENEID_CONFIG_PATH").unwrap_or(String::from("/etc/aeneid/config.toml"));
    let config: Config = toml::from_str(&*fs::read_to_string(config_path).expect("read config"))
        .expect("parse config");
    assert_eq!(
        config.cfg_version, 1,
        "incompatible cfg_version, please update aeneid"
    );

    if let Some(u) = args.value_of("username") {
        let username = unix_to_github(u.to_string()).expect("invalid username");
        if access::check_allowed(&*username, config) {
            let keys = keys::get_keys(&*username);
            println!("{}", keys);
        } else {
            std::process::exit(1);
        }
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
fn unix_to_github(unix: String) -> Result<String, Box<dyn std::error::Error>> {
    let prefixed = Regex::new(r"^_[a-zA-Z0-9]*$").unwrap();
    let normal = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();

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
