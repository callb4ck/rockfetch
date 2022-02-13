use std::env::var;
use crate::exec;

/// An enum containing the supported operating systems and exceptions
pub enum OS {
    Arch,
    Artix,
    Ubuntu,
    Void,
    None,
    Other(String),
}

/// Get the currently running operating system
pub fn get_os() -> OS {
    if let Ok(os_release) = rs_release::get_os_release() {
        if let Some(os) = os_release.get("ID") {
            match os.as_str() {
                "arch" => return OS::Arch,
                "artix" => return OS::Artix,
                "ubuntu" => return OS::Ubuntu,
                "void" => return OS::Void,
                _ => {}
            }
        }

        if let Some(os_name) = os_release.get("NAME") {
            return OS::Other(os_name.clone());
        }
    }

    OS::None
}

/// Get the hostname of the machine
pub fn get_host() -> String {
    let mut host = std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| std::env::var("HOSTNAME").unwrap_or_else(|_| String::from("")))
        .strip_suffix('\n')
        .unwrap_or("")
        .to_string();

    if host.is_empty() {
        host = var("HOSTNAME").unwrap_or_else(|_| String::from(""));
    }

    host
}

/// Get the username
#[inline]
pub fn get_user() -> String {
    var("USER").unwrap_or_else(|_| exec!("whoami"))
}
