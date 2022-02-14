use crate::{
    exec,
    get_info::{shell::{get_gui, get_shell}, system::{get_host, get_user, get_uptime}}, env_or, rgb,
};

pub fn print() {
    let reset = env_or!(reset);
    let c1 = env_or!("C1" or rgb!(99, 99, 249));
    let c2 = env_or!("C2" or rgb!(45, 45, 252));
    let font1 = env_or!("FONT1" or rgb!(43, 43, 242));
    let font2 = env_or!("FONT2" or rgb!(255, 85, 255));
    let font3 = env_or!("FONT3" or rgb!(255, 255, 255));


    let user = get_user();

    let host = get_host();

    let kernel = exec!("uname", "-sr");
    let uptime = get_uptime();
    let packages = exec!(notrim "rpm", "-qa").matches('\n').count();
    let shell = get_shell();

    let gui = get_gui();

    println!(r" {c1}      ______
 {c1}     /   __ \   {font2}{user}{font3}@{font2}{host}
 {c1}     |  |  \ \  {font1}OS:{reset}        Fedora
 {c1}  ___!  !__/ /  {font1}KERNEL:{reset}    {kernel}
 {c1} / __, {c2} ,___/   {font1}UPTIME:{reset}    {uptime}
 {c1}/ / {c2} |  |       {font1}PACKAGES:{reset}  {packages}
 {c2}\ \__!  |       {font1}SHELL:{reset}     {shell}
 {c2} \______/       {font1}DE/WM:{reset}     {gui}{reset}
")
}
