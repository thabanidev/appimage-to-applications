use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppRequest {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub category: String,
}
