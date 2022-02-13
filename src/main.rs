use get_info::system::{get_os, OS};
use os_functions::{arch, artix, void, ubuntu};

mod get_info;
mod os_functions;
mod settings;

fn main() {
    match get_os() {
        OS::Arch => arch::print(),
        OS::Artix => artix::print(),
        OS::Ubuntu=> ubuntu::print(),
        OS::Void=> void::print(),

        _ => {}
    };
}
