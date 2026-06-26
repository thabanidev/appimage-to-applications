use std::path::Path;

pub fn parse_version_from_path(path: &str) -> Option<String> {
    let filename = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())?;
    parse_version_from_filename(filename)
}

pub fn parse_version_from_filename(filename: &str) -> Option<String> {
    let mut name = filename.to_string();

    for ext in [".AppImage", ".appimage", ".APPIMAGE"] {
        if let Some(stripped) = name.strip_suffix(ext) {
            name = stripped.to_string();
            break;
        }
    }

    for marker in ["_v", "-v", "_V", "-V"] {
        if let Some(idx) = name.find(marker) {
            let rest = &name[idx + marker.len()..];
            if let Some(version) = extract_version_prefix(rest) {
                return Some(version);
            }
        }
    }

    for part in name.split(['-', '_']) {
        if let Some(version) = extract_version_prefix(part) {
            if part.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                return Some(version);
            }
        }
    }

    None
}

pub fn resolve_app_version(
    desktop_version: Option<&str>,
    app_folder: &Path,
    exec_path: &Path,
) -> Option<String> {
    if let Some(version) = desktop_version.filter(|value| !value.is_empty()) {
        return Some(version.to_string());
    }

    if let Some(version) = version_from_folder_filenames(app_folder) {
        return Some(version);
    }

    if let Some(filename) = exec_path.file_name().and_then(|name| name.to_str()) {
        if let Some(version) = parse_version_from_filename(filename) {
            return Some(version);
        }
    }

    None
}

fn version_from_folder_filenames(folder: &Path) -> Option<String> {
    let entries = std::fs::read_dir(folder).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let filename = path.file_name()?.to_str()?;
        if let Some(version) = parse_version_from_filename(filename) {
            return Some(version);
        }
    }

    None
}

fn extract_version_prefix(value: &str) -> Option<String> {
    let mut end = 0usize;

    for (index, character) in value.char_indices() {
        if character.is_ascii_digit() || character == '.' {
            end = index + character.len_utf8();
            continue;
        }

        if end > 0 {
            break;
        }
    }

    if end == 0 {
        return None;
    }

    let version = value[..end].trim_matches('.');
    if version.is_empty() || !version.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }

    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_godot_appimage_version() {
        assert_eq!(
            parse_version_from_filename("Godot_v4.5.AppImage"),
            Some("4.5".to_string())
        );
    }

    #[test]
    fn parses_lmms_appimage_version() {
        assert_eq!(
            parse_version_from_filename("lmms-1.2.2-x86_64.AppImage"),
            Some("1.2.2".to_string())
        );
    }
}
