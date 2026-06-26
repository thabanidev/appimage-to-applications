use crate::linux::app_name::parse_app_name_from_path;

#[tauri::command]
pub fn parse_app_name(path: String) -> Result<String, String> {
    if path.trim().is_empty() {
        return Err("AppImage path is required".to_string());
    }
    Ok(parse_app_name_from_path(&path))
}
