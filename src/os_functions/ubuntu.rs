use crate::{
    env_or, exec,
    get_info::{
        shell::{get_gui, get_shell},
        system::{get_host, get_uptime, get_user},
    }, rgb,
};

pub fn print() {
    let reset = env_or!(reset);
    let c1 = env_or!("C1" or rgb!(226, 78, 78));
    let c2 = env_or!("C2" or rgb!(192, 103, 9));
    let font1 = env_or!("FONT1" or rgb!(226, 78, 78));
    let font2 = env_or!("FONT2" or rgb!(192, 103, 9));
    let font3 = env_or!("FONT3" or rgb!(226, 78, 78));

    let user = get_user();

    let host = get_host();

    let kernel = exec!("uname", "-sr");
    let uptime = get_uptime();
    let packages = exec!(notrim "dpkg", "--get-selections")
        .matches('\n')
        .count();
    let shell = get_shell();

    let gui = get_gui();

    print!(
        "           {c1}_      {font2}{user}{font3}@{font2}{host}
     {c2},----{c1}(_)     {font1}OS:{reset}        Ubuntu
   {c1}_{c2}/  ---  \\     {font1}KERNEL:{reset}    {kernel}
  {c1}(_) {c2}|   |  |    {font1}UPTIME:{reset}    {uptime}
    {c2}\\  --- {c1}_{c2}/     {font1}PACKAGES:{reset}  {packages}
     {c2}`----{c1}(_)     {font1}SHELL:{reset}     {shell}
                  {font1}DE/WM:{reset}     {gui}

"
    );
}
