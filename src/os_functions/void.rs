use crate::{
    env_or, exec,
    get_info::{
        shell::{get_gui, get_shell},
        system::{get_host, get_uptime, get_user},
    }, rgb,
};

pub fn print() {
    let reset = env_or!(reset);
    let c1 = env_or!("C1" or rgb!(85, 255, 85));
    let c2 = env_or!("C2" or rgb!(171, 194, 171));
    let font1 = env_or!("FONT1" or rgb!(85, 255, 85));
    let font2 = env_or!("FONT2" or rgb!(255, 255, 85));
    let font3 = env_or!("FONT3" or rgb!(255, 255, 255));

    let user = get_user();

    let host = get_host();

    let kernel = exec!("uname", "-sr");
    let uptime = get_uptime();
    let packages = exec!(notrim "xbps-query", "-l").matches('\n').count();
    let shell = get_shell();

    let gui = get_gui();

    println!(
        "     {c1}_______      {font2}{user}{font3}@{font2}{host}
  {c1}_ \\______ -     {font1}OS:{reset}        Void
 {c1}| \\  {c2}___{c1}  \\ |    {font1}KERNEL:{reset}    {kernel}
 {c1}| | {c2}/   \\{c1} | |    {font1}UPTIME:{reset}    {uptime}
 {c1}| | {c2}\\___/{c1} | |    {font1}PACKAGES:{reset}  {packages}
 {c1}| \\______ \\_|    {font1}SHELL:{reset}     {shell}
  {c1}-_______\\       {font1}DE/WM:{reset}     {gui}{reset}
"
    );
}
