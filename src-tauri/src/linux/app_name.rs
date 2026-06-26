use std::path::Path;

pub fn parse_app_name_from_path(path: &str) -> String {
    let filename = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(path);
    parse_app_name_from_filename(filename)
}

pub fn parse_app_name_from_filename(filename: &str) -> String {
    let mut name = filename.to_string();

    for ext in [".AppImage", ".appimage", ".APPIMAGE"] {
        if let Some(stripped) = name.strip_suffix(ext) {
            name = stripped.to_string();
            break;
        }
    }

    name = strip_version_suffix(&name);
    title_case_words(&name)
}

fn strip_version_suffix(name: &str) -> String {
    let markers = ["_v", "-v", "_V", "-V"];
    for marker in markers {
        if let Some(idx) = name.find(marker) {
            let rest = &name[idx + marker.len()..];
            if rest.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                return name[..idx].to_string();
            }
        }
    }

    for suffix in ["-stable", "_stable", "-beta", "_beta", "-alpha", "_alpha"] {
        if let Some(stripped) = name.strip_suffix(suffix) {
            return stripped.to_string();
        }
    }

    name.to_string()
}

fn title_case_words(input: &str) -> String {
    input
        .split(|c| c == '_' || c == '-' || c == ' ')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut result = first.to_uppercase().collect::<String>();
                    result.push_str(&chars.as_str().to_lowercase());
                    result
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_godot_versioned_name() {
        assert_eq!(
            parse_app_name_from_filename("Godot_v4.5.AppImage"),
            "Godot"
        );
    }

    #[test]
    fn parses_cursor_name() {
        assert_eq!(parse_app_name_from_filename("cursor.AppImage"), "Cursor");
    }
}
