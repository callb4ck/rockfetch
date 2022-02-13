use crate::{
    exec,
    get_info::{shell::{get_gui, get_shell}, system::{get_host, get_user}},
    settings::*,
};

pub fn print() {
    let user = get_user();

    let host = get_host();

    let kernel = exec!("uname", "-sr");
    let uptime = exec!("uptime", "-p").chars().skip(3).collect::<String>();
    let packages = exec!(notrim "xbps-query", "-l").matches('\n').count();
    let shell = get_shell();

    let gui = get_gui();

    println!(
        "      {C1}_______      {FONT2}{user}{FONT1}@{FONT2}{host}
   {C1}_ \\______ -     {FONT1}OS:{RESET}        Void
  {C1}| \\  {C2}___{C1}  \\ |    {FONT1}KERNEL:{RESET}    {kernel}
  {C1}| | {C2}/   \\{C1} | |    {FONT1}UPTIME:{RESET}    {uptime}
  {C1}| | {C2}\\___/{C1} | |    {FONT1}PACKAGES:{RESET}  {packages}
  {C1}| \\______ \\_|    {FONT1}SHELL:{RESET}     {shell}
   {C1}-_______\\       {FONT1}DE/WM:{RESET}     {gui}{RESET}
");

}
