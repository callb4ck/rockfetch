use std::fs::read_to_string;

use toml::Value;

fn main() {
    let config_file = read_to_string("Config.toml")
        .unwrap_or_else(|_| panic!("\n\n\n[ERROR]: Config.toml file not found\n\n\n"))
        .parse::<Value>()
        .unwrap_or_else(|e| panic!("\n\n\n[ERROR]: Parsing error:\n{e}\n\n\n"));

    let mut reset: Option<&Value> = None;

    let mut c1: Option<&Value> = None;
    let mut c2: Option<&Value> = None;

    let mut font1: Option<&Value> = None;
    let mut font2: Option<&Value> = None;
    let mut font3: Option<&Value> = None;

    match config_file.get("conf") {
        Some(conf_table) => {
            if let Some(theme_name) = conf_table.get("theme") {
                match config_file.get("theme") {
                    Some(theme_table) => match theme_table.get(theme_name.as_str().unwrap()) {
                        Some(theme) => {
                            reset = theme.get("RESET");

                            c1 = theme.get("COLOR1");
                            c2 = theme.get("COLOR2");

                            font1 = theme.get("FONT1");
                            font2 = theme.get("FONT2");
                            font3 = theme.get("FONT3");
                        },
                        _ => panic!("\n\n\n[ERROR]: Undefined theme '{theme}'.\n(Perhaps a [theme.{theme}] is missing)\n\n\n", theme=theme_name.as_str().unwrap())
                    }

                    _ => panic!("\n\n\n[ERROR]: Undefined theme '{theme}'.\n(Perhaps a [theme.{theme}] is missing)\n\n\n", theme=theme_name.as_str().unwrap())

                }
            }

            macro_rules! set_env {
                ($get:expr; $var:expr => $val:expr) => {
                    match conf_table.get($get) {
                        Some(v) => {
                            println!("cargo:rustc-env={}={}", $var, v.as_str().unwrap());
                        }

                        _ => {
                            if let Some(v) = $val {
                                println!("cargo:rustc-env={}={}", $var, v.as_str().unwrap());
                            }
                        }
                    }
                };
            }

            set_env!("RESET"; "RESET" => reset);

            set_env!("COLOR1"; "C1" => c1);
            set_env!("COLOR2"; "C2" => c2);

            set_env!("FONT1"; "FONT1" => font1);
            set_env!("FONT2"; "FONT2" => font2);
            set_env!("FONT3"; "FONT3" => font3);
        }
        _ => panic!("\n\n\n[ERROR]: 'conf' table is not defined .\n(Try adding a [conf])\n\n\n"),
    }
}
