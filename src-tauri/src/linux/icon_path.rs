use std::fs;
use std::path::{Path, PathBuf};

use super::install_layout::ICON_FILE_STEM;

const IMAGE_EXTENSIONS: &[&str] = &["png", "svg", "webp", "jpg", "jpeg", "ico", "xpm"];

pub fn resolve_icon_path(desktop_icon: &str, app_folder: &str, exec_path: &str) -> String {
    if let Some(path) = existing_file_path(desktop_icon) {
        return path;
    }

    if app_folder.is_empty() {
        return String::new();
    }

    let folder = Path::new(app_folder);

    if let Some(file_name) = Path::new(desktop_icon)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
    {
        let candidate = folder.join(file_name);
        if let Some(path) = existing_file_path(&candidate.to_string_lossy()) {
            return path;
        }
    }

    find_icon_in_folder(folder, exec_path).unwrap_or_default()
}

fn find_icon_in_folder(folder: &Path, exec_path: &str) -> Option<String> {
    let entries = fs::read_dir(folder).ok()?;
    let mut images = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && is_image_file(&path) {
            if path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .is_some_and(|stem| stem == ICON_FILE_STEM)
            {
                return Some(path.to_string_lossy().to_string());
            }
            images.push(path);
        }
    }

    if images.is_empty() {
        return None;
    }

    let exec_stem = Path::new(exec_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let folder_stem = folder
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    for image in &images {
        if image
            .file_stem()
            .is_some_and(|stem| stem.eq_ignore_ascii_case("icon"))
        {
            return Some(image.to_string_lossy().to_string());
        }
    }

    for stem in [exec_stem, folder_stem] {
        if stem.is_empty() {
            continue;
        }

        for image in &images {
            if image
                .file_stem()
                .is_some_and(|name| name.eq_ignore_ascii_case(stem))
            {
                return Some(image.to_string_lossy().to_string());
            }
        }
    }

    if images.len() == 1 {
        return Some(images[0].to_string_lossy().to_string());
    }

    images.sort_by_key(|path| path.file_name().map(|name| name.to_ascii_lowercase()));
    images
        .first()
        .map(|path| path.to_string_lossy().to_string())
}

fn existing_file_path(path: &str) -> Option<String> {
    if path.is_empty() {
        return None;
    }

    let candidate = PathBuf::from(path);
    if candidate.is_file() {
        return Some(candidate.to_string_lossy().to_string());
    }

    None
}

fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            IMAGE_EXTENSIONS
                .iter()
                .any(|allowed| ext.eq_ignore_ascii_case(allowed))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("{name}-{unique}"));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn finds_icon_when_desktop_points_to_missing_standard_name() {
        let folder = temp_dir("godot-icon");
        let exec = folder.join("Godot");
        fs::write(&exec, b"exec").expect("write exec");
        fs::write(folder.join("godot.png"), b"png").expect("write icon");

        let resolved = resolve_icon_path(
            &folder.join("icon.png").to_string_lossy(),
            &folder.to_string_lossy(),
            &exec.to_string_lossy(),
        );

        assert_eq!(resolved, folder.join("godot.png").to_string_lossy());
        let _ = fs::remove_dir_all(folder);
    }

    #[test]
    fn prefers_icon_png_when_present() {
        let folder = temp_dir("icon-png");
        let exec = folder.join("App");
        fs::write(&exec, b"exec").expect("write exec");
        fs::write(folder.join("icon.png"), b"png").expect("write icon");
        fs::write(folder.join("app.png"), b"png").expect("write alt");

        let resolved = resolve_icon_path("", &folder.to_string_lossy(), &exec.to_string_lossy());

        assert_eq!(resolved, folder.join("icon.png").to_string_lossy());
        let _ = fs::remove_dir_all(folder);
    }
}
