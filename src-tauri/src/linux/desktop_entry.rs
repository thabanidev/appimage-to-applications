use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::categories::{normalize_category, DEFAULT_CATEGORY};
use super::paths::{dock_verified_key, managed_desktop_key};

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub name: String,
    pub comment: String,
    pub exec: String,
    pub icon: String,
    pub version: Option<String>,
    pub categories: String,
    pub startup_wm_class: Option<String>,
    pub dock_verified: Option<bool>,
    pub managed: bool,
}

#[derive(Debug, Clone)]
pub struct DesktopEntryWrite<'a> {
    pub name: &'a str,
    pub comment: &'a str,
    pub exec: &'a str,
    pub icon: &'a str,
    pub version: Option<&'a str>,
    pub categories: &'a str,
    pub startup_wm_class: Option<&'a str>,
    pub dock_verified: Option<bool>,
    pub managed: bool,
}

pub fn write_desktop_file(path: &Path, entry: DesktopEntryWrite<'_>) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let categories = normalize_category(entry.categories)?;

    let mut content = String::from(
        "[Desktop Entry]\n\
         Type=Application\n",
    );
    content.push_str(&format!("Name={}\n", escape_desktop_value(entry.name)));
    content.push_str(&format!("Comment={}\n", escape_desktop_value(entry.comment)));
    content.push_str(&format!("Exec={}\n", escape_desktop_value(entry.exec)));
    content.push_str(&format!("Icon={}\n", escape_desktop_value(entry.icon)));
    if let Some(version) = entry.version.filter(|value| !value.is_empty()) {
        content.push_str(&format!("Version={}\n", escape_desktop_value(version)));
    }
    content.push_str("Terminal=false\n");
    content.push_str(&format!("Categories={categories};\n"));

    if let Some(wm_class) = entry.startup_wm_class.filter(|value| !value.is_empty()) {
        content.push_str(&format!(
            "StartupWMClass={}\n",
            escape_desktop_value(wm_class)
        ));
    }

    if entry.managed {
        content.push_str(&format!("{}=true\n", managed_desktop_key()));
    }

    if let Some(verified) = entry.dock_verified {
        content.push_str(&format!(
            "{}={}\n",
            dock_verified_key(),
            if verified { "true" } else { "false" }
        ));
    }

    fs::write(path, content).map_err(|e| e.to_string())
}

pub fn needs_dock_fix(managed: bool, dock_verified: Option<bool>) -> bool {
    managed && dock_verified == Some(false)
}

pub fn parse_desktop_file(path: &Path) -> Result<DesktopEntry, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut values: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            values.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    let managed = values
        .get(managed_desktop_key())
        .map(|v| v == "true")
        .unwrap_or(false);

    let categories = values
        .get("Categories")
        .map(|value| value.trim_end_matches(';').to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_CATEGORY.to_string());

    let startup_wm_class = values.get("StartupWMClass").cloned();
    let dock_verified = parse_dock_verified(&values, &startup_wm_class);

    Ok(DesktopEntry {
        name: values.get("Name").cloned().unwrap_or_default(),
        comment: values.get("Comment").cloned().unwrap_or_default(),
        exec: values.get("Exec").cloned().unwrap_or_default(),
        icon: values.get("Icon").cloned().unwrap_or_default(),
        version: values.get("Version").cloned(),
        categories,
        startup_wm_class,
        dock_verified,
        managed,
    })
}

fn parse_dock_verified(
    values: &HashMap<String, String>,
    startup_wm_class: &Option<String>,
) -> Option<bool> {
    match values.get(dock_verified_key()).map(String::as_str) {
        Some("true") => Some(true),
        Some("false") => Some(false),
        _ => {
            if startup_wm_class
                .as_ref()
                .is_some_and(|value| !value.is_empty())
            {
                // Launchers configured manually (for example Godot) are treated as verified.
                Some(true)
            } else {
                Some(false)
            }
        }
    }
}

fn escape_desktop_value(value: &str) -> String {
    value.replace('\\', "\\\\")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grandfather_verified_when_startup_wm_class_exists() {
        let mut values = HashMap::new();
        values.insert("StartupWMClass".to_string(), "Godot".to_string());

        assert_eq!(parse_dock_verified(&values, &Some("Godot".to_string())), Some(true));
    }

    #[test]
    fn explicit_false_means_needs_fix() {
        let mut values = HashMap::new();
        values.insert(dock_verified_key().to_string(), "false".to_string());

        assert_eq!(parse_dock_verified(&values, &Some("LMMS".to_string())), Some(false));
        assert!(needs_dock_fix(true, Some(false)));
    }
}
