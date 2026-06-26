use crate::linux::categories::normalize_category;
use crate::linux::desktop_entry::{parse_desktop_file, write_desktop_file, DesktopEntryWrite};
use crate::linux::desktop_scan::find_desktop_file;
use crate::linux::refresh::refresh_desktop_database;
use crate::linux::startup_wm_class::derive_startup_wm_class;
use crate::models::command_result::CommandResult;
use crate::models::update_app_request::UpdateAppRequest;
use std::path::Path;

#[tauri::command]
pub fn update_installed_app(request: UpdateAppRequest) -> Result<CommandResult, String> {
    let slug = request.slug.trim();
    let name = request.name.trim();

    if slug.is_empty() {
        return Ok(CommandResult::err("Application slug is required"));
    }
    if name.is_empty() {
        return Ok(CommandResult::err("Application name is required"));
    }

    let desktop_path = find_desktop_file(slug)?;
    let entry = parse_desktop_file(&desktop_path)?;
    let category = normalize_category(&request.category)?;

    let startup_wm_class = derive_startup_wm_class(
        name,
        Path::new(&entry.exec),
        Path::new(&entry.exec),
    );

    write_desktop_file(
        &desktop_path,
        DesktopEntryWrite {
            name,
            comment: request.description.trim(),
            exec: &entry.exec,
            icon: &entry.icon,
            version: entry.version.as_deref(),
            categories: &category,
            startup_wm_class: Some(&startup_wm_class),
            managed: entry.managed,
        },
    )?;

    refresh_desktop_database();

    Ok(CommandResult::ok(
        format!("Updated {name}"),
        vec![
            format!("Updated launcher for {name}"),
            "Refreshed applications menu".to_string(),
        ],
    ))
}
