use std::path::Path;

use crate::linux::app_version::parse_version_from_path;
use crate::linux::categories::normalize_category;
use crate::linux::desktop_entry::{write_desktop_file, DesktopEntryWrite};
use crate::linux::fs_ops::{chmod_executable, copy_file, ensure_dir, file_exists};
use crate::linux::install_layout::copy_icon_to_standard_path;
use crate::linux::paths::{app_folder, desktop_file, executable_filename, slugify, standard_icon_filename};
use crate::linux::refresh::refresh_desktop_database;
use crate::models::command_result::CommandResult;
use crate::models::install_request::InstallRequest;

#[tauri::command]
pub fn install_appimage(request: InstallRequest) -> Result<CommandResult, String> {
    let name = request.name.trim();
    if name.is_empty() {
        return Ok(CommandResult::err("Application name is required"));
    }

    let app_image_path = Path::new(&request.app_image_path);
    let icon_path = Path::new(&request.icon_path);

    if !app_image_path.exists() {
        return Ok(CommandResult::err("AppImage file does not exist"));
    }
    if !icon_path.exists() {
        return Ok(CommandResult::err("Icon file does not exist"));
    }

    let slug = slugify(name);
    if slug.is_empty() {
        return Ok(CommandResult::err(
            "Application name must contain at least one letter or number",
        ));
    }

    let folder = app_folder(name)?;
    let executable_dest = folder.join(executable_filename(name));
    let icon_dest = folder.join(standard_icon_filename());
    let desktop_path = desktop_file(&slug)?;

    if file_exists(&folder) {
        return Ok(CommandResult::err(format!(
            "Application folder already exists: {}",
            folder.display()
        )));
    }
    if desktop_path.exists() {
        return Ok(CommandResult::err(format!(
            "Desktop launcher already exists: {}",
            desktop_path.display()
        )));
    }

    let mut log = Vec::new();

    ensure_dir(&folder).map_err(|e| e.to_string())?;
    log.push(format!("Created folder {}", folder.display()));

    copy_file(app_image_path, &executable_dest).map_err(|e| e.to_string())?;
    log.push(format!("Copied application to {}", executable_filename(name)));

    chmod_executable(&executable_dest).map_err(|e| e.to_string())?;
    log.push("Made application executable".to_string());

    copy_icon_to_standard_path(icon_path, &folder)?;
    log.push("Copied icon to icon.png".to_string());

    let category = normalize_category(&request.category)?;
    let version = parse_version_from_path(&request.app_image_path);

    write_desktop_file(
        &desktop_path,
        DesktopEntryWrite {
            name,
            comment: request.description.trim(),
            exec: &executable_dest.to_string_lossy(),
            icon: &icon_dest.to_string_lossy(),
            version: version.as_deref(),
            categories: &category,
            startup_wm_class: Some(name),
            dock_verified: Some(false),
            managed: true,
        },
    )?;
    log.push(format!("Created desktop launcher {}", desktop_path.display()));

    refresh_desktop_database();
    log.push("Refreshed applications menu".to_string());

    Ok(CommandResult::ok(
        format!("{name} was installed successfully"),
        log,
    ))
}
