//! Freedesktop main categories for .desktop files.
//! https://specifications.freedesktop.org/menu-spec/latest/category-registry.html

pub const DEFAULT_CATEGORY: &str = "Utility";

pub const ALLOWED_CATEGORIES: &[&str] = &[
    "AudioVideo",
    "Development",
    "Education",
    "Game",
    "Graphics",
    "Network",
    "Office",
    "Science",
    "Settings",
    "System",
    "Utility",
];

pub fn normalize_category(category: &str) -> Result<String, String> {
    let trimmed = category.trim();
    if trimmed.is_empty() {
        return Ok(DEFAULT_CATEGORY.to_string());
    }

    let matched = ALLOWED_CATEGORIES
        .iter()
        .find(|allowed| allowed.eq_ignore_ascii_case(trimmed));

    matched
        .map(|value| (*value).to_string())
        .ok_or_else(|| {
            format!(
                "Invalid category '{trimmed}'. Choose one of: {}",
                ALLOWED_CATEGORIES.join(", ")
            )
        })
}

pub fn suggest_category(name: &str) -> String {
    let lower = name.to_lowercase();

    if contains_any(
        &lower,
        &[
            "lmms", "audacity", "spotify", "vlc", "ffmpeg", "ardour", "reaper",
        ],
    ) {
        return "AudioVideo".to_string();
    }

    if contains_any(
        &lower,
        &[
            "godot", "unity", "unreal", "blender", "gimp", "inkscape", "krita",
        ],
    ) {
        return "Graphics".to_string();
    }

    if contains_any(
        &lower,
        &["code", "cursor", "vscode", "idea", "dev", "git", "docker"],
    ) {
        return "Development".to_string();
    }

    if contains_any(&lower, &["game", "steam", "minecraft", "lutris"]) {
        return "Game".to_string();
    }

    if contains_any(&lower, &["firefox", "chrome", "browser", "discord", "slack"]) {
        return "Network".to_string();
    }

    if contains_any(&lower, &["libreoffice", "onlyoffice", "writer", "calc"]) {
        return "Office".to_string();
    }

    DEFAULT_CATEGORY.to_string()
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggests_audio_for_lmms() {
        assert_eq!(suggest_category("LMMS"), "AudioVideo");
    }

    #[test]
    fn normalizes_category() {
        assert_eq!(normalize_category("game").unwrap(), "Game");
    }
}
