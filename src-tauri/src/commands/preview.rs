use std::path::Path;

use crate::linux::install_layout::ICON_FILE_NAME;
use crate::linux::paths::{
    app_folder, desktop_file, executable_filename, slugify, standard_icon_filename,
};
use crate::models::install_preview::{InstallPreview, InstallPreviewStep};

#[tauri::command]
pub fn preview_install(
    app_image_path: String,
    icon_path: String,
    name: String,
) -> Result<InstallPreview, String> {
    validate_preview_inputs(&app_image_path, &icon_path, &name)?;

    let slug = slugify(&name);
    if slug.is_empty() {
        return Err("Application name must contain at least one letter or number".to_string());
    }

    let folder = app_folder(&name)?;
    let app_image_file = executable_filename(&name);
    let icon_file = standard_icon_filename().to_string();
    let desktop = desktop_file(&slug)?;

    let steps = vec![
        InstallPreviewStep {
            label: "Create folder".to_string(),
            path: folder.to_string_lossy().to_string(),
        },
        InstallPreviewStep {
            label: "Copy application".to_string(),
            path: app_image_file.clone(),
        },
        InstallPreviewStep {
            label: "Copy icon".to_string(),
            path: ICON_FILE_NAME.to_string(),
        },
    InstallPreviewStep {
            label: "Set StartupWMClass".to_string(),
            path: name.clone(),
        },
        InstallPreviewStep {
            label: "Create desktop launcher".to_string(),
            path: desktop.to_string_lossy().to_string(),
        },
    ];

    Ok(InstallPreview {
        app_folder: folder.to_string_lossy().to_string(),
        app_image_file,
        icon_file,
        desktop_file: desktop.to_string_lossy().to_string(),
        steps,
    })
}

fn validate_preview_inputs(
    app_image_path: &str,
    icon_path: &str,
    name: &str,
) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Application name is required".to_string());
    }
    if !Path::new(app_image_path).exists() {
        return Err("AppImage file does not exist".to_string());
    }
    if !Path::new(icon_path).exists() {
        return Err("Icon file does not exist".to_string());
    }
    validate_icon_extension(icon_path)?;
    Ok(())
}

fn validate_icon_extension(icon_path: &str) -> Result<(), String> {
    let ext = Path::new(icon_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if matches!(ext.as_str(), "png" | "svg" | "webp" | "jpg" | "jpeg") {
        Ok(())
    } else {
        Err("Icon must be PNG, SVG, WebP, or JPEG".to_string())
    }
}
