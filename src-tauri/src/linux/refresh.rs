use std::process::Command;

use super::paths::desktop_dir;

pub fn refresh_desktop_database() {
    let desktop_path = match desktop_dir() {
        Ok(path) => path,
        Err(_) => return,
    };

    let _ = Command::new("update-desktop-database")
        .arg(desktop_path)
        .status();
}
