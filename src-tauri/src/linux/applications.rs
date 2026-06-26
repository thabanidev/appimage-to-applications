use std::path::Path;

use super::paths::{applications_root, home_dir};

pub fn is_under_applications(path: &str) -> bool {
    let Ok(root) = applications_root() else {
        return false;
    };

    let target = Path::new(path);
    target.starts_with(&root)
        || target.starts_with(
            home_dir()
                .unwrap_or_default()
                .join("Applications"),
        )
}
