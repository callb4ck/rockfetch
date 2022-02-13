use crate::{
    exec,
    get_info::{shell::{get_gui, get_shell}, system::{get_host, get_user, get_uptime}},
    settings::*,
};

pub fn print() {
    let user = get_user();

    let host = get_host();

    let kernel = exec!("uname", "-sr");
    let uptime = get_uptime();
    let packages = exec!(notrim "dpkg", "--get-selections").matches('\n').count();
    let shell = get_shell();

    let gui = get_gui();

        print!(
"           {C1}_      {FONT2}{user}{FONT1}@{FONT2}{host}
     ,----{C1}(_)     {FONT1}OS:{RESET}        Ubuntu
   {C1}_{C2}/  ---  \\     {FONT1}KERNEL:{RESET}    {kernel}
  {C1}(_) {C2}|   |  |    {FONT1}UPTIME:{RESET}    {uptime}
    {C2}\\  --- {C1}_{C2}/     {FONT1}PACKAGES:{RESET}  {packages}
     {C2}`----{C1}(_)     {FONT1}SHELL:{RESET}     {shell}
                  {FONT1}DE/WM:{RESET}     {gui}

");
}
