use std::path::Path;

use crate::linux::applications::is_under_applications;
use crate::linux::desktop_entry::parse_desktop_file;
use crate::linux::desktop_scan::find_desktop_file;
use crate::linux::fs_ops::remove_path;
use crate::linux::paths::{applications_root, slugify};
use crate::linux::refresh::refresh_desktop_database;
use crate::models::command_result::CommandResult;

#[tauri::command]
pub fn remove_installed_app(slug: String) -> Result<CommandResult, String> {
    let slug = slug.trim();
    if slug.is_empty() {
        return Ok(CommandResult::err("Application slug is required"));
    }

    let mut log = Vec::new();

    if let Ok(path) = find_desktop_file(slug) {
        let entry = parse_desktop_file(&path)?;

        if !entry.managed && !is_under_applications(&entry.exec) {
            return Ok(CommandResult::err(
                "This desktop entry is outside ~/Applications and cannot be removed here",
            ));
        }

        let app_name = entry.name.clone();
        let app_folder = Path::new(&entry.exec)
            .parent()
            .map(|parent| parent.to_path_buf())
            .ok_or_else(|| {
                "Could not determine application folder from desktop entry".to_string()
            })?;

        if app_folder.exists() {
            remove_path(&app_folder).map_err(|e| e.to_string())?;
            log.push(format!("Removed folder {}", app_folder.display()));
        }

        remove_path(&path).map_err(|e| e.to_string())?;
        log.push(format!("Removed desktop launcher {}", path.display()));

        refresh_desktop_database();
        log.push("Refreshed desktop launcher database".to_string());

        return Ok(CommandResult::ok(format!("Removed {app_name}"), log));
    }

    let root = applications_root()?;
    for entry in std::fs::read_dir(&root).map_err(|e| e.to_string())?.flatten() {
        let folder = entry.path();
        if !folder.is_dir() {
            continue;
        }

        let folder_name = folder
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();

        if slugify(folder_name) != slug {
            continue;
        }

        remove_path(&folder).map_err(|e| e.to_string())?;
        log.push(format!("Removed folder {}", folder.display()));

        return Ok(CommandResult::ok(
            format!("Removed {folder_name}"),
            log,
        ));
    }

    Ok(CommandResult::err(format!(
        "Could not find an installed application matching '{slug}'"
    )))
}
