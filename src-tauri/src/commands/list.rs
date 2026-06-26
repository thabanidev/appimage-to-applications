use crate::linux::desktop_scan::scan_installed_apps;
use crate::models::installed_app::InstalledApp;

#[tauri::command]
pub fn list_installed_apps() -> Result<Vec<InstalledApp>, String> {
    scan_installed_apps()
}
