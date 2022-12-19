# Rockfetch

A fetch script for completely aestethic purposes.

# Configure
The configurations happens at compile time, simply edit the file: `Config.toml`.

# Compile
* Install Cargo and Rust (at least `1.58.0`). The preferred way to do so is trough [rustup](https://rustup.rs/).
* Execute `cargo build --release`
* You will find the built executable in `target/release/`. Feel free to strip it with `strip target/release/rockfetch`

# Currently supported operating systems

## Linux
* Arch | Endeavour
* Artix
* Fedora
* Ubuntu
* Void

# Note: Fedora
Since version 0.1.6, to count packages on Fedora, rockfetch will attempt to read the `/var/cache/dnf/packages.db` database with rusqlite (sqlite3).

This is default behaviour.

If you'd prefer rockfetch to behave in the old way, calling the `rpm` command instead of reading DNF's package database, disable the `fedora-sqlite` feature
by compiling with the flag: `--no-default-features` or by editing the  `Cargo.toml` file manually (by removing `"fedora-sqlite"` from `default = [...]`).
