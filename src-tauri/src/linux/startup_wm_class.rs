use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use super::app_name::parse_app_name_from_filename;
use super::desktop_entry::parse_desktop_file;

/// Best-effort StartupWMClass for an AppImage install so the dock uses one icon.
pub fn derive_startup_wm_class(
    display_name: &str,
    executable_path: &Path,
    app_image_source: &Path,
) -> String {
    if let Some(from_appimage) = try_startup_wm_class_from_appimage(app_image_source) {
        return from_appimage;
    }

    fallback_startup_wm_class(display_name, executable_path, app_image_source)
}

fn fallback_startup_wm_class(
    display_name: &str,
    executable_path: &Path,
    app_image_source: &Path,
) -> String {
    if let Some(stem) = executable_path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
    {
        if !stem.contains(' ') {
            return stem.to_string();
        }

        if let Some(first) = stem.split_whitespace().next().filter(|part| !part.is_empty()) {
            return first.to_string();
        }
    }

    if let Some(first) = display_name
        .split_whitespace()
        .next()
        .filter(|part| !part.is_empty())
    {
        return first.to_string();
    }

    if let Some(filename) = app_image_source.file_name().and_then(|name| name.to_str()) {
        let parsed = parse_app_name_from_filename(filename);
        if let Some(first) = parsed.split_whitespace().next().filter(|part| !part.is_empty()) {
            return first.to_string();
        }
    }

    display_name.replace(' ', "")
}

fn try_startup_wm_class_from_appimage(app_image: &Path) -> Option<String> {
    if !app_image.is_file() {
        return None;
    }

    let extract_parent = std::env::temp_dir().join(format!(
        "appimage-to-applications-extract-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_nanos()
    ));

    fs::create_dir_all(&extract_parent).ok()?;
    let extract_result = extract_startup_wm_class(app_image, &extract_parent);
    let _ = fs::remove_dir_all(&extract_parent);
    extract_result
}

fn extract_startup_wm_class(app_image: &Path, extract_parent: &Path) -> Option<String> {
    let status = Command::new(app_image)
        .arg("--appimage-extract")
        .current_dir(extract_parent)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;

    if !status.success() {
        return None;
    }

    find_startup_wm_class_in_tree(&extract_parent.join("squashfs-root"))
}

fn find_startup_wm_class_in_tree(dir: &Path) -> Option<String> {
    if !dir.is_dir() {
        return None;
    }

    let mut stack = vec![dir.to_path_buf()];

    while let Some(current) = stack.pop() {
        let entries = fs::read_dir(&current).ok()?;

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                stack.push(path);
                continue;
            }

            if path.extension().and_then(|ext| ext.to_str()) != Some("desktop") {
                continue;
            }

            let parsed = parse_desktop_file(&path).ok()?;
            if let Some(wm_class) = parsed
                .startup_wm_class
                .filter(|value| !value.is_empty())
            {
                return Some(wm_class);
            }
        }
    }

    None
}

pub fn preview_startup_wm_class(
    display_name: &str,
    executable_path: &Path,
    app_image_source: &Path,
) -> String {
    fallback_startup_wm_class(display_name, executable_path, app_image_source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_single_word_executable_name() {
        let class = fallback_startup_wm_class(
            "Godot Engine",
            Path::new("/home/user/Applications/Godot/Godot"),
            Path::new("/tmp/Godot_v4.5.AppImage"),
        );

        assert_eq!(class, "Godot");
    }

    #[test]
    fn uses_first_word_when_executable_has_spaces() {
        let class = fallback_startup_wm_class(
            "Godot Engine",
            Path::new("/home/user/Applications/Godot Engine/Godot Engine"),
            Path::new("/tmp/godot.AppImage"),
        );

        assert_eq!(class, "Godot");
    }

    #[test]
    fn falls_back_to_executable_name() {
        let class = fallback_startup_wm_class(
            "",
            Path::new("/home/user/Applications/LMMS/LMMS"),
            Path::new("/tmp/lmms-1.2.2-x86_64.AppImage"),
        );

        assert_eq!(class, "LMMS");
    }
}
