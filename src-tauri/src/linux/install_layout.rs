use std::fs;
use std::path::{Path, PathBuf};

pub const ICON_FILE_STEM: &str = "icon";

pub fn executable_filename(name: &str) -> String {
    name.to_string()
}

pub fn icon_filename_for_source(icon_source: &Path) -> String {
    let extension = icon_source
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| !ext.is_empty())
        .unwrap_or("png");

    format!("{ICON_FILE_STEM}.{extension}")
}

pub fn copy_icon_to_app_folder(
    icon_source: &Path,
    folder: &Path,
) -> Result<PathBuf, String> {
    let dest = folder.join(icon_filename_for_source(icon_source));

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::copy(icon_source, &dest).map_err(|e| e.to_string())?;
    Ok(dest)
}
