use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallRequest {
    pub app_image_path: String,
    pub icon_path: String,
    pub name: String,
    pub description: String,
    pub category: String,
}
