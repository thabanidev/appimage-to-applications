use crate::linux::desktop_entry::{parse_desktop_file, write_desktop_file, DesktopEntryWrite};
use crate::linux::desktop_scan::find_desktop_file;
use crate::linux::refresh::refresh_desktop_database;
use crate::linux::wm_class::detect_wm_class_for_exec;
use crate::models::command_result::CommandResult;

#[tauri::command]
pub fn fix_dock_grouping(slug: String) -> Result<CommandResult, String> {
    let slug = slug.trim();
    if slug.is_empty() {
        return Ok(CommandResult::err("Application slug is required"));
    }

    let desktop_path = find_desktop_file(slug)?;
    let entry = parse_desktop_file(&desktop_path)?;

    if entry.exec.is_empty() {
        return Ok(CommandResult::err("Desktop launcher has no Exec path"));
    }

    let previous = entry.startup_wm_class.clone().unwrap_or_default();
    let mut log = vec![format!("Launching {} to detect its window class...", entry.name)];

    let wm_class = detect_wm_class_for_exec(&entry.exec, &entry.name)?;
    log.push(format!("Using StartupWMClass: {wm_class}"));

    write_desktop_file(
        &desktop_path,
        DesktopEntryWrite {
            name: &entry.name,
            comment: &entry.comment,
            exec: &entry.exec,
            icon: &entry.icon,
            version: entry.version.as_deref(),
            categories: &entry.categories,
            startup_wm_class: Some(&wm_class),
            dock_verified: Some(true),
            managed: entry.managed,
        },
    )?;

    log.push(format!("Updated {}", desktop_path.display()));

    refresh_desktop_database();
    log.push("Refreshed applications menu".to_string());

    if previous == wm_class {
        log.push("Dock grouping was already set to this value.".to_string());
    }

    log.push("Quit the app completely, then open it again from your applications menu.".to_string());

    Ok(CommandResult::ok(
        format!("Updated dock grouping for {}", entry.name),
        log,
    ))
}
