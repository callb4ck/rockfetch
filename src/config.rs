
#[macro_export]
macro_rules! env_or {
    ($var:literal or $default:expr) => {
        match option_env!($var) {
            Some(v) => v,
            _ => $default,
        }
    };

    (reset) => {
        match option_env!("RESET") {
            Some(v) => v,
            _ => "\x1B[0m",
        }
    }
}

#[macro_export]
macro_rules! rgb {
    ($r:literal, $g:literal, $b:literal) => {
        concat!("\x1B[1;38;2;", $r, ";", $g, ";", $b, "m")
    };
}
