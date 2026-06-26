use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallPreview {
    pub app_folder: String,
    pub app_image_file: String,
    pub icon_file: String,
    pub desktop_file: String,
    pub steps: Vec<InstallPreviewStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallPreviewStep {
    pub label: String,
    pub path: String,
}
