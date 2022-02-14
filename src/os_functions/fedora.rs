use crate::{
    exec,
    get_info::{shell::{get_gui, get_shell}, system::{get_host, get_user, get_uptime}},
    config::*,
};

pub fn print() {
    let user = get_user();

    let host = get_host();

    let kernel = exec!("uname", "-sr");
    let uptime = get_uptime();
    let packages = exec!(notrim "rpm", "-qa").matches('\n').count();
    let shell = get_shell();

    let gui = get_gui();

    println!(r"{C1}      ______
{C1}     /   __ \   {FONT2}{user}{FONT1}@{FONT2}{host}
{C1}     |  |  \ \  {FONT1}OS:{RESET}        Fedora
{C1}  ___!  !__/ /  {FONT1}KERNEL:{RESET}    {kernel}
{C1} / __, {C2} ,___/   {FONT1}UPTIME:{RESET}    {uptime}
{C1}/ / {C2} |  |       {FONT1}PACKAGES:{RESET}  {packages}
{C2}\ \__!  |       {FONT1}SHELL:{RESET}     {shell}
{C2} \______/       {FONT1}DE/WM:{RESET}     {gui}{RESET}
")
}
