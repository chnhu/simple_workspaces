use crate::models::user::User;
use crate::schema::{WorkspaceSchema, WorkspaceDetailSchema};
use sqlx::{PgPool, Result};

pub async fn get_user_by_id(
    pool: &PgPool,
    user_id: i32,
) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM \"user\" WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_workspace_by_id(
    pool: &PgPool,
    workspace_id: i32,
    user_id: i32,
) -> Result<Option<WorkspaceDetailSchema>> {
    let workspace = sqlx::query_as::<_, WorkspaceDetailSchema>(
    "
            SELECT
                w.workspace_id,
                w.workspace_name,
                w.description,
                ws.status_name as status,
                w.updated_at
            FROM
                workspace w
            INNER JOIN
                workspace_status_master ws
                USING (workspace_status_id)
            WHERE
                w.workspace_id = $1
                and w.user_id = $2
        ",
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(workspace)
}

pub async fn list_all_workspaces(pool: &PgPool, user_id: i32) -> Result<Vec<WorkspaceSchema>> {
    let workspaces = sqlx::query_as::<_, WorkspaceSchema>(
    "
            SELECT
                w.workspace_id,
                w.workspace_name,
                w.description,
                ws.status_name as status
            FROM
                workspace w
            INNER JOIN
                workspace_status_master ws
                USING (workspace_status_id)
            WHERE
                w.user_id = $1
            ORDER BY
                w.workspace_id
        "
    )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(workspaces)
}

pub async fn create_workspace(
    pool: &PgPool,
    name: &str,
    description: Option<String>,
    workspace_status_id: i32,
    user_id: i32,
) -> Result<i32> {
    let workspace_id = sqlx::query!(
            "INSERT INTO workspace (workspace_name, description, workspace_status_id, user_id) VALUES ($1, $2, $3, $4) RETURNING workspace_id",
            name,
            description,
            workspace_status_id,
            user_id,
        )
        .fetch_one(pool)
        .await?
        .workspace_id;

    Ok(workspace_id)
}

pub async fn update_workspace_status(
    pool: &PgPool,
    workspace_id: i32,
    new_status: i32,
) -> Result<()> {
    sqlx::query!(
            "UPDATE workspace SET workspace_status_id = $1, updated_at = current_timestamp WHERE workspace_id = $2",
            new_status,
            workspace_id
        )
        .execute(pool)
        .await?;

    Ok(())
}
