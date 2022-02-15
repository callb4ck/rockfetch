use crate::{
    env_or, exec,
    get_info::{
        shell::{get_gui, get_shell},
        system::{get_host, get_uptime, get_user},
    }, rgb,
};

pub fn print() {
    let reset = env_or!(reset);
    let c1 = env_or!("C1" or rgb!(127, 186, 255));
    let c2 = env_or!("C2" or rgb!(127, 127, 255));
    let c3 = env_or!("C3" or rgb!(255, 127, 127));

    let font1 = env_or!("FONT1" or rgb!(127, 186, 255));
    let font2 = env_or!("FONT2" or rgb!(127, 63, 191));
    let font3 = env_or!("FONT3" or rgb!(127, 186, 255));

    let user = get_user();

    let host = get_host();

    let kernel = exec!("uname", "-sr");
    let uptime = get_uptime();
    let packages = exec!(notrim "pacman", "-Q").matches('\n').count();
    let shell = get_shell();

    let gui = get_gui();

    println!(
r"       {c3}/{c1}\         {font2}{user}{font3}@{font2}{host}
     {c3}/{c1}/  \{c2}\       {font1}OS:{reset}        EndeavourOS
    {c3}/{c1}/    \ {c2}\     {font1}KERNEL:{reset}    {kernel}
  {c3}/ {c1}/     _) {c2})    {font1}UPTIME:{reset}    {uptime}
 {c3}/_{c1}/___-- {c2}__-     {font1}PACKAGES:{reset}  {packages}
  {c2}/____--         {font1}SHELL:{reset}     {shell}
                  {font1}DE/WM:{reset}     {gui}
{reset}")
}
