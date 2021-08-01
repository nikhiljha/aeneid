# aeneid

<img src="https://user-images.githubusercontent.com/2773700/127755383-955cdd1f-8f8a-4c9a-b6a0-8203729ce4b8.png" align="right"
     alt="aeneid-meme" width="300" height="300">
     
GitHub's "teams" feature is basically a free, zero-ops [IdP](https://en.wikipedia.org/wiki/Identity_provider). Let's use it to authenticate to OpenSSH! You _probably_ shouldn't use this in production, but I can't stop you.

## What / How?

1. GitHub provides an API to ensure that a given user is in a given team within a given org. GitHub also provides an endpoint (`/username.keys`) to retrieve someone's SSH keys.
2. OpenSSHd provides a way to execute an arbitrary binary before user login, and then reads its stdout to grab SSH public keys. Failing that, it falls back to `authorized_keys`. Learn more by running `man sshd_config`.
3. Glue them together and you get this project.

## Installation

Install aeneid with your favorite package manager. Since I released this project 0 minutes ago, it's not available as a package yet. Check back later for packages, or compile it yourself (`cargo build --release` then copy the binary to `/usr/local/bin` or something).

## Configuration

The configuration lives in `/etc/aeneid/config.toml`. If it doesn't exist, create it based on the `config.toml` in this repository. All fields should have comments explaining what they do.

You'll also need to set `AuthorizedKeysCommand /path/to/bin/aeneid` and `AuthorizedKeysCommandRunAs aeneid` in your sshd_config (typically `/etc/ssh/sshd_config`) so that OpenSSH knows where to get keys from.

## Usage

If your GitHub username starts with a number, prefix your username with an `_` to login. Otherwise, your username is your GitHub username. See the `unix_to_github` function in `main.rs` for more information.

Automatically creating users is currently unsupported, you'll need to create the corresponding user manually before logging in (`adduser username`).

```bash
$ ssh username@example.com # that's it
```

## Security

I've thought about security a little, but not nearly as much as I'd like. **I don't recommend using this anywhere security is important.** It's your responsibility to ensure that...

- `/etc/aeneid` and all children are owned by a separate user (call it `aeneid`) and set with restrictive permissions (`chmod 600`)
- `AuthorizedKeysCommandRunAs` is set to the separate user that owns `/etc/aeneid`
- all relevant GitHub accounts are kept secure (MFA, good passwords, etc.)
- other problematic SSH config options (e.x. password auth) are disabled
- your SSH keys are not compromised
- possibly other things I haven't thought of

## FAQ

**Why did you make this?**

I **really** didn't want to set up LDAP. I **really really** didn't want to set up LDAP. I **really really really really really really** didn't want to set up LDAP. In the end I set up LDAP, so hopefully this is useful to someone else.

**What's with the name?**

I thought it was silly. You're accepting a present (free, zero-ops IdP), but in the process, GitHub *could* silently swap out the public keys it returns and authenticate to your machines. So if you squint: trojan horse.
