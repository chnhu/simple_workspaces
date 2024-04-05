use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateWorkspaceSchema {
    #[serde(rename = "workspaceName")]
    pub workspace_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "workspaceStatusId")]
    pub workspace_status_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateWorkspaceSchema {
    #[serde(rename = "workspaceId")]
    pub workspace_id: i32,
    #[serde(rename = "workspaceStatusId")]
    pub workspace_status_id: i32,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct WorkspaceSchema {
    #[serde(rename = "workspaceId")]
    pub workspace_id: i32,
    #[serde(rename = "workspaceName")]
    pub workspace_name: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct WorkspaceDetailSchema {
    #[serde(rename = "workspaceId")]
    pub workspace_id: i32,
    #[serde(rename = "workspaceName")]
    pub workspace_name: String,
    pub description: Option<String>,
    pub status: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}