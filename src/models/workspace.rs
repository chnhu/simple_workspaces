use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Workspace {
    #[serde(rename = "workspaceId")]
    pub workspace_id: i32,
    #[serde(rename = "workspaceName")]
    pub workspace_name: String,
    pub description: String,
    #[serde(rename = "workspaceStatusId")]
    pub workspace_status_id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
