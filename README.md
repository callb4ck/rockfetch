# Rockfetch

A fetch script for completely aestethic purposes.

# Installation

## Manual compilation
This is the preferred method, as it allows for configuration.

* Install Cargo and Rust (at least `1.58.0`). The preferred way to do so is trough [rustup](https://rustup.rs/).
* Execute `cargo build --release`
* You will find the built executable in `target/release/`. Feel free to strip it with `strip target/release/rockfetch`

## Using cargo install
This method is simpler, yet I would suggest the use manual compilation as it allows for configuration, while using cargo install you are forced with the defaults.

With this method simply run the command `cargo install rockfetch`, and you're all set.

# Configuration
The configurations happens at compile time, simply edit the file: `Config.toml`. The configuration is always stored in the binary

# Currently supported operating systems

## Linux
* Arch | Endeavour
* Artix
* Fedora
* Ubuntu
* Void

# Note: Fedora
Since version `0.1.6`, to count packages on Fedora, rockfetch will attempt to read the `/var/cache/dnf/packages.db` database with rusqlite (sqlite3).

Since version `0.1.7`, to count packages on Fedora, rockfetch will attempt to read the `/var/lib/rpm/rpmdb.sqlite` database with rusqlite (sqlite3).

This is default behaviour.

If you'd prefer rockfetch to behave in the old way, calling the `rpm` command instead of directly reading RPM's package database, disable the `fedora-sqlite` feature
by compiling with the flag: `--no-default-features` or by editing the  `Cargo.toml` file manually (removing `"fedora-sqlite"` from `default = [...]`).
