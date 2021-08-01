use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    process::Command,
};

use regex::Regex;
use serde_derive::Deserialize;

/// config file in /etc/aeneid/configuration.toml
#[derive(Deserialize)]
pub struct Config {
    /// Version of the aeneid configuration
    pub(crate) cfg_version: u32,

    /// GitHub org name (https://github.com/organizations/new)
    pub(crate) org: Option<String>,
    /// GitHub team name (https://github.com/orgs/<org>/teams)
    pub(crate) team: Option<String>,
    /// GitHub access token with read:org (https://github.com/settings/tokens)
    pub(crate) token: Option<String>,

    /// set of GitHub users to always allow
    pub(crate) overrides: HashMap<String, String>,
}

/// Initializes aeneid on a typical Linux system
pub(crate) fn init() {
    // function is written like C and not like rust because it interfaces with the
    // OS so much
    assert!(cfg!(target_os = "linux"), "init only works on linux");
    assert!(
        !fs::metadata("/etc/aeneid/.noinit").is_ok(),
        "your distro has disabled --init"
    );

    // create aeneid user
    Command::new("useradd")
        .args(&["aeneid"])
        .output()
        .expect("create user");

    // configure aeneid
    fs::create_dir_all("/etc/aeneid").expect("create config directory");
    if !fs::metadata("/etc/aeneid/config.toml").is_ok() {
        fs::write("/etc/aeneid/config.toml", include_bytes!("config.toml"))
            .expect("create config file");
        Command::new("chown")
            .args(&["-R", "aeneid:aeneid", "/etc/aeneid"])
            .output()
            .expect("change config dir owner");
        Command::new("chmod")
            .args(&["-R", "700", "/etc/aeneid"])
            .output()
            .expect("change config dir owner");
    } else {
        eprintln!("configuration file already exists, not creating...");
    }

    // configure sshd
    let mut file = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .open("/etc/ssh/sshd_config")
        .expect("open sshd config");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("read sshd config");

    let cmd_configured = Regex::new(r"\nAuthorizedKeysCommand").unwrap();
    let user_configured = Regex::new(r"\nAuthorizedKeysCommandUser").unwrap();
    if !cmd_configured.is_match(&*contents) && !user_configured.is_match(&*contents) {
        file.write(
            format!(
                "AuthorizedKeysCommand {}\nAuthorizedKeysCommandUser aeneid",
                std::env::current_exe()
                    .expect("get executable")
                    .to_str()
                    .expect("get path as str"),
            )
            .as_bytes(),
        )
        .expect("write sshd config");
    } else {
        eprintln!("sshd appears to already be configured, not modifying...");
    }
}
