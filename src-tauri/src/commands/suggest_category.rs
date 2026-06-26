use crate::linux::categories::suggest_category;

#[tauri::command]
pub fn suggest_app_category(name: String) -> String {
    suggest_category(&name)
}
