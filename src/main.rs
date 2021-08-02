use std::{fs, process::exit};

use clap::App;
use config::{init, Config};

mod access;
mod config;
mod keys;

fn main() {
    let args = App::new("aeneid")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Nikhil Jha <mail@nikhiljha.com>")
        .about("authenticate SSH users with their GitHub authorized_keys")
        .arg("-i, --init 'Initializes aeneid'")
        .arg(clap::Arg::new("username")
            .conflicts_with("init")
            .required(true)
            .about("a unix username")
        )
        .get_matches();

    if args.is_present("init") {
        init();
        exit(0);
    }

    // TODO: security: (footgun) validate that the config has chmod 600
    // TODO: security: (footgun) validate that AuthorizedKeysCommandRunAs is set
    let config_path =
        std::env::var("AENEID_CONFIG_PATH").unwrap_or(String::from("/etc/aeneid/config.toml"));
    let config: Config = toml::from_str(&*fs::read_to_string(config_path).expect("read config"))
        .expect("parse config");
    assert_eq!(
        config.cfg_version, 2,
        "incompatible cfg_version, please update aeneid"
    );

    if let Some(unix_name) = args.value_of("username") {
        assert!(unix_name.is_ascii(), "username must be ascii");

        let github_name = access::unix_to_github(unix_name).unwrap();
        if config.overrides.contains_key(&*unix_name) {
            print_keys(config.overrides.get(unix_name).unwrap());
        } else if access::check_allowed(&*github_name, config) {
            print_keys(&*github_name);
        } else {
            exit(1);
        }
    }
}

/// print keys of a given GitHub username
fn print_keys(github_name: &str) {
    let keys = keys::get_keys(github_name);
    println!("{}", keys);
}
