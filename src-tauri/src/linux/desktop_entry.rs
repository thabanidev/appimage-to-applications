use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::categories::{normalize_category, DEFAULT_CATEGORY};
use super::paths::managed_desktop_key;

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub name: String,
    pub comment: String,
    pub exec: String,
    pub icon: String,
    pub version: Option<String>,
    pub categories: String,
    pub startup_wm_class: Option<String>,
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

    fs::write(path, content).map_err(|e| e.to_string())
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
    let desktop_stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default();

    Ok(DesktopEntry {
        name: resolve_display_name(&values, desktop_stem),
        comment: values.get("Comment").cloned().unwrap_or_default(),
        exec: values.get("Exec").cloned().unwrap_or_default(),
        icon: values.get("Icon").cloned().unwrap_or_default(),
        version: values.get("Version").cloned(),
        categories,
        startup_wm_class,
        managed,
    })
}

pub fn resolve_display_name(values: &HashMap<String, String>, desktop_stem: &str) -> String {
    let name = values.get("Name").map(String::as_str).unwrap_or_default();
    let generic = values
        .get("GenericName")
        .map(String::as_str)
        .unwrap_or_default();

    if looks_like_technical_id(name) {
        if !generic.is_empty() {
            return generic.to_string();
        }
        return humanize_desktop_stem(desktop_stem);
    }

    if !name.is_empty() {
        return name.to_string();
    }

    for (key, value) in values {
        if key.starts_with("Name[") && !value.is_empty() {
            return value.clone();
        }
    }

    if !generic.is_empty() {
        return generic.to_string();
    }

    humanize_desktop_stem(desktop_stem)
}

fn looks_like_technical_id(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() {
        return false;
    }

    value.contains('.')
        && !value.contains(' ')
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '-' || ch == '_')
}

fn humanize_desktop_stem(stem: &str) -> String {
    let segment = stem.rsplit('.').next().unwrap_or(stem);
    let mut chars = segment.chars();
    let Some(first) = chars.next() else {
        return stem.to_string();
    };

    first
        .to_uppercase()
        .chain(chars)
        .collect::<String>()
        .replace('-', " ")
        .replace('_', " ")
}

fn escape_desktop_value(value: &str) -> String {
    value.replace('\\', "\\\\")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_generic_name_for_technical_ids() {
        let mut values = HashMap::new();
        values.insert("Name".to_string(), "com.usebottles.bottles".to_string());
        values.insert("GenericName".to_string(), "Bottles".to_string());

        assert_eq!(
            resolve_display_name(&values, "com.usebottles.bottles"),
            "Bottles"
        );
    }

    #[test]
    fn humanizes_desktop_stem_when_name_missing() {
        let values = HashMap::new();
        assert_eq!(
            resolve_display_name(&values, "com.usebottles.bottles"),
            "Bottles"
        );
    }
}
