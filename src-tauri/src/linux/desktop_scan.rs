use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::installed_app::InstalledApp;

use super::applications::is_under_applications;
use super::app_version::resolve_app_version;
use super::desktop_entry::parse_desktop_file;
use super::icon_path::resolve_icon_path;
use super::install_layout::ICON_FILE_NAME;
use super::paths::{applications_root, desktop_dir, slugify};

pub fn scan_installed_apps() -> Result<Vec<InstalledApp>, String> {
    let mut apps = list_from_desktop_files()?;
    let known_folders: HashSet<String> = apps.iter().map(|app| app.app_folder.clone()).collect();
    apps.extend(list_orphan_application_folders(&known_folders)?);

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(apps)
}

pub fn find_desktop_file(slug: &str) -> Result<PathBuf, String> {
    let path = desktop_dir()?.join(format!("{slug}.desktop"));
    if !path.exists() {
        return Err(format!("Desktop file not found for '{slug}'"));
    }
    Ok(path)
}

fn list_from_desktop_files() -> Result<Vec<InstalledApp>, String> {
    let dir = desktop_dir()?;
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut apps = Vec::new();

    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(Vec::new()),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("desktop") {
            continue;
        }

        let parsed = match parse_desktop_file(&path) {
            Ok(parsed) => parsed,
            Err(_) => continue,
        };

        if !parsed.managed && !is_under_applications(&parsed.exec) {
            continue;
        }

        apps.push(build_installed_app_from_path(&path)?);
    }

    Ok(apps)
}

fn build_installed_app_from_path(path: &Path) -> Result<InstalledApp, String> {
    let parsed = parse_desktop_file(path)?;
    let path_string = path.to_string_lossy().to_string();

    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string();

    let app_folder = exec_parent_folder(&parsed.exec);
    let exec_path = Path::new(&parsed.exec);
    let folder_path = Path::new(&app_folder);

    Ok(InstalledApp {
        slug,
        name: parsed.name,
        description: parsed.comment,
        exec_path: parsed.exec.clone(),
        icon_path: resolve_icon_path(&parsed.icon, &app_folder, &parsed.exec),
        version: resolve_app_version(parsed.version.as_deref(), folder_path, exec_path),
        app_folder,
        desktop_file: path_string,
        categories: parsed.categories,
        managed: parsed.managed,
        has_desktop_file: true,
    })
}

fn list_orphan_application_folders(
    known_folders: &HashSet<String>,
) -> Result<Vec<InstalledApp>, String> {
    let root = applications_root()?;
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut apps = Vec::new();

    for entry in fs::read_dir(&root).map_err(|e| e.to_string())?.flatten() {
        let folder = entry.path();
        if !folder.is_dir() {
            continue;
        }

        let folder_string = folder.to_string_lossy().to_string();
        if known_folders.contains(&folder_string) {
            continue;
        }

        let Some(executable) = find_executable_in_folder(&folder) else {
            continue;
        };

        let folder_name = folder
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Application")
            .to_string();

        let icon_path = resolve_icon_path(
            "",
            &folder_string,
            &executable.to_string_lossy(),
        );

        apps.push(InstalledApp {
            slug: slugify(&folder_name),
            name: folder_name.clone(),
            description: String::new(),
            exec_path: executable.to_string_lossy().to_string(),
            icon_path,
            version: resolve_app_version(None, &folder, &executable),
            app_folder: folder_string,
            desktop_file: String::new(),
            categories: String::new(),
            managed: false,
            has_desktop_file: false,
        });
    }

    Ok(apps)
}

fn find_executable_in_folder(folder: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(folder).ok()?;
    let mut fallback = None;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("AppImage"))
        {
            return Some(path);
        }

        let file_name = path.file_name()?.to_str()?;
        if file_name == ICON_FILE_NAME {
            continue;
        }

        if fallback.is_none() {
            fallback = Some(path);
        }
    }

    fallback
}

fn exec_parent_folder(exec: &str) -> String {
    Path::new(exec)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}
