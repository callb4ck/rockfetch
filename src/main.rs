use get_info::system::{get_os, OS};
use os_functions::{arch, artix, void, ubuntu, fedora, generic_unix};

mod get_info;
mod os_functions;
mod config;

fn main() {
    match get_os() {
        OS::Arch   => arch::print(),
        OS::Artix  => artix::print(),
        OS::Fedora => fedora::print(),
        OS::Ubuntu => ubuntu::print(),
        OS::Void   => void::print(),
        OS::GenericUnix(v) => generic_unix::print(&v),
        _ => {}
    };
}
