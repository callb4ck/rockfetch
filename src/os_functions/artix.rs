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
    let packages = exec!(notrim "pacman", "-Q").matches('\n').count();
    let shell = get_shell();

    let gui = get_gui();

    println!(
        r"{C1}      /\         {FONT2}{user}{FONT1}@{FONT2}{host}
{C1}     /  \        {FONT1}OS:        {RESET}Artix
{C1}    /`'.,\       {FONT1}KERNEL:    {RESET}{kernel}
{C1}   /  {C2}   ',      {FONT1}UPTIME:    {RESET}{uptime}
{C2}  /      ,`\     {FONT1}PACKAGES:  {RESET}{packages}
{C2} /   ,.'`.  \    {FONT1}SHELL:     {RESET}{shell}
{C2}/.,'`     `'.\   {FONT1}DE/WM:     {RESET}{gui}
{RESET}",
    );
}
