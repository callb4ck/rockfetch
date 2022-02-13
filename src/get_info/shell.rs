use super::command::basename;

macro_rules! unwrap_return {
    ($var:expr) => {
        if let Ok(v) = std::env::var($var) {
            return v;
        }
    };

    ($var:expr => $ret:expr) => {
        if let Ok(_) = std::env::var($var) {
            return $ret.to_string();
        }
    }
}

pub fn get_gui() -> String {

    unwrap_return!("GNOME_DESKTOP_SESSION_ID" => "GNOME");
    unwrap_return!("MATE_DESKTOP_SESSION_ID" => "MATE");
    unwrap_return!("TDE_FULL_SESSION" => "Trinity");

    unwrap_return!("DESKTOP_SESSION");
    unwrap_return!("XDG_CURRENT_DESKTOP");

    String::new()
}

#[inline]
pub fn get_shell() -> String {
    basename(&std::env::var("SHELL").unwrap_or_else(|_| String::new()))
}
