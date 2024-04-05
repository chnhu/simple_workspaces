use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct WorkspaceStatusMaster {
    #[serde(rename = "workspaceStatusId")]
    pub workspace_status_id: i32,
    #[serde(rename = "statusName")]
    pub status_name: String,
}