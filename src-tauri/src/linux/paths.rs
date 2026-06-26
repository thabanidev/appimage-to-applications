use std::path::{Path, PathBuf};

const MANAGED_DESKTOP_KEY: &str = "X-AppImage-To-Applications";
const DOCK_VERIFIED_KEY: &str = "X-AppImage-DockVerified";

pub fn home_dir() -> Result<PathBuf, String> {
    std::env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| "Could not resolve home directory".to_string())
}

pub fn applications_root() -> Result<PathBuf, String> {
    Ok(home_dir()?.join("Applications"))
}

pub fn app_folder(name: &str) -> Result<PathBuf, String> {
    let folder = applications_root()?.join(name);
    validate_under_applications(&folder)?;
    Ok(folder)
}

pub fn desktop_dir() -> Result<PathBuf, String> {
    Ok(home_dir()?
        .join(".local")
        .join("share")
        .join("applications"))
}

pub fn desktop_file(slug: &str) -> Result<PathBuf, String> {
    let path = desktop_dir()?.join(format!("{slug}.desktop"));
    validate_under_desktop_dir(&path)?;
    Ok(path)
}

pub fn slugify(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn executable_filename(name: &str) -> String {
    crate::linux::install_layout::executable_filename(name)
}

pub fn standard_icon_filename() -> &'static str {
    crate::linux::install_layout::ICON_FILE_NAME
}

#[allow(dead_code)]
pub fn app_image_filename(name: &str) -> String {
    format!("{name}.AppImage")
}

#[allow(dead_code)]
pub fn icon_filename(slug: &str, icon_source: &Path) -> String {
    let _ = slug;
    let ext = icon_source
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    format!("icon.{ext}")
}

pub fn managed_desktop_key() -> &'static str {
    MANAGED_DESKTOP_KEY
}

pub fn dock_verified_key() -> &'static str {
    DOCK_VERIFIED_KEY
}

fn validate_under_applications(path: &Path) -> Result<(), String> {
    if path.components().any(|c| c.as_os_str() == "..") {
        return Err("Invalid application path".to_string());
    }
    Ok(())
}

fn validate_under_desktop_dir(path: &Path) -> Result<(), String> {
    if path.components().any(|c| c.as_os_str() == "..") {
        return Err("Invalid desktop file path".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_handles_spaces() {
        assert_eq!(slugify("Godot Engine"), "godot-engine");
    }
}
