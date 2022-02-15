use crate::exec;
use std::{
    env::var,
    fs::File,
    io::{BufRead, BufReader},
};

/// An enum containing the supported operating systems and exceptions
pub enum OS {
    Arch,
    Artix,
    Endeavour,
    Fedora,
    Ubuntu,
    Void,
    None,
    GenericUnix(String),
}

/// Get the currently running operating system
pub fn get_os() -> OS {
    let kernel = exec!("uname", "-s");

    if kernel.ends_with("BSD") {
        return OS::GenericUnix("BSD".to_string());
    } else if kernel.starts_with("GNU") {
        return OS::GenericUnix("Linux".to_string());
    }

    match kernel.as_str() {
        "Darwin" => return OS::GenericUnix("MacOS".to_string()),
        "DragonFly" | "Bitrig" => return OS::GenericUnix("BSD".to_string()),
        _ => {}
    }

    if let Ok(os_release) = rs_release::get_os_release() {
        if let Some(os) = os_release.get("ID") {
            match os.as_str() {
                "arch" => return OS::Arch,
                "artix" => return OS::Artix,
                "endeavouros" => return OS::Endeavour,
                "fedora" => return OS::Fedora,
                "ubuntu" => return OS::Ubuntu,
                "void" => return OS::Void,
                _ => {}
            }
        }

        if let Some(os_name) = os_release.get("NAME") {
            return OS::GenericUnix(os_name.clone());
        }
    }

    OS::None
}

/// Get the hostname of the machine
pub fn get_host() -> String {

    let mut host = var("HOSTNAME").unwrap_or_else(|_| 
        std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| String::new())
        .trim_end()
        .to_string()
    );

    if host.is_empty() {
        host = std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| String::new())
        .trim_end()
        .to_string()
    }

    host
}

/// Get the username
#[inline]
pub fn get_user() -> String {
    var("USER").unwrap_or_else(|_| exec!("whoami"))
}

/// Get current uptime
pub fn get_uptime() -> String {
    let file = File::open("/proc/uptime").unwrap();

    let mut buf: Vec<u8> = Vec::new();

    BufReader::new(file)
        .read_until(b'.', &mut buf)
        .unwrap();

    buf.pop();

    let mut minutes = String::from_utf8(buf).unwrap().parse::<u32>().unwrap() / 60;

    let hours = minutes / 60;

    minutes -= hours * 60;

    format!(
        "{hours} hour{}, {minutes} minute{}",
        if hours != 1 { "s" } else { "" },
        if minutes != 1 { "s" } else { "" }
    )
}
