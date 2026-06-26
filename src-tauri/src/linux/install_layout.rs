use std::fs;
use std::path::Path;

pub const ICON_FILE_NAME: &str = "icon.png";

pub fn executable_filename(name: &str) -> String {
    name.to_string()
}

pub fn copy_icon_to_standard_path(icon_source: &Path, folder: &Path) -> Result<std::path::PathBuf, String> {
    let dest = folder.join(ICON_FILE_NAME);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::copy(icon_source, &dest).map_err(|e| e.to_string())?;
    Ok(dest)
}
