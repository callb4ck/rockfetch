

/// Executes a system command
/// (notrim does the same thing but doesn't trim the output)
#[macro_export]
macro_rules! exec {
    ($cmd:expr) => {
        String::from_utf8(
            std::process::Command::new($cmd)
            .output()
            .unwrap()
            .stdout
        ).unwrap().trim().to_string()
    };

    ($cmd:expr, $($arg:expr),*) => {
        String::from_utf8(
            std::process::Command::new($cmd)
            $(
                .arg($arg)
            )*
            .output()
            .unwrap()
            .stdout
        ).unwrap().trim().to_string()
    };

    (notrim $cmd:expr) => {
        String::from_utf8(
            std::process::Command::new($cmd)
            .output()
            .unwrap()
            .stdout
        ).unwrap()
    };

    (notrim $cmd:expr, $($arg:expr),*) => {
        String::from_utf8(
            std::process::Command::new($cmd)
            $(
                .arg($arg)
            )*
            .output()
            .unwrap()
            .stdout
        ).unwrap()
    };
}

/// Simulates the basename command
pub fn basename(input: &str) -> String {
    if let Some(input) = input.split('/').last() {
        return input.trim_end().to_string()
    }

    exec!("basename", input)
}
