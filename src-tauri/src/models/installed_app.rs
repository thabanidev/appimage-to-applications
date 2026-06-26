use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledApp {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub exec_path: String,
    pub icon_path: String,
    pub version: Option<String>,
    pub app_folder: String,
    pub desktop_file: String,
    pub categories: String,
    pub managed: bool,
    pub has_desktop_file: bool,
}
