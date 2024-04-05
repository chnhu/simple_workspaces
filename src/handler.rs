use axum::{
    extract::{Path, State, FromRequestParts},
    http::{
        StatusCode,
        request::Parts,
    },
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::{repository, schema};

pub struct ExtractUserId(i32);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for ExtractUserId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_id) = parts.headers.get("user-id") {
            if let Some(user_id_str) = user_id.to_str().ok() {
                if let Some(user_id_value) = user_id_str.parse::<i32>().ok() {
                    Ok(ExtractUserId(user_id_value.clone()))
                } else {
                    Err((StatusCode::BAD_REQUEST, "`user-id` header is invalid"))
                }
            } else {
                Err((StatusCode::BAD_REQUEST, "`user-id` header is invalid"))
            }
        } else {
            Err((StatusCode::BAD_REQUEST, "`user-id` header is missing"))
        }
    }
}

pub async fn get_workspace_by_id_handler(
    ExtractUserId(user_id): ExtractUserId,
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {

    match repository::get_workspace_by_id(&pool, id, user_id).await {
        Ok(workspace) => {
            if let Some(workspace) = workspace {
                Json(workspace).into_response()
            } else {
                (StatusCode::NOT_FOUND, "Not found").into_response()
            }
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn list_all_workspace_hanlder(
    ExtractUserId(user_id): ExtractUserId,
    State(pool): State<PgPool>,
) -> impl IntoResponse {

    match repository::list_all_workspaces(&pool, user_id).await {
        Ok(workspaces) => {
            Json(workspaces).into_response()
        },
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn create_workspace_handler(
    ExtractUserId(user_id): ExtractUserId,
    State(pool): State<PgPool>,
    Json(body): Json<schema::CreateWorkspaceSchema>,
) -> impl IntoResponse {
    match repository::get_user_by_id(&pool, user_id).await {
        Ok(_) => {
            match repository::create_workspace(
                &pool, &body.workspace_name, body.description, body.workspace_status_id, user_id
            ).await {
                Ok(workspace_id) => {
                    Json(workspace_id).into_response()
                },
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}

pub async fn update_workspace_handler(
    ExtractUserId(user_id): ExtractUserId,
    State(pool): State<PgPool>,
    Json(body): Json<schema::UpdateWorkspaceSchema>,
) -> impl IntoResponse {

    match repository::get_user_by_id(&pool, user_id).await {
        Ok(_) => {
            match repository::update_workspace_status(
                &pool, body.workspace_id, body.workspace_status_id
            ).await {
                Ok(workspace_id) => {
                    Json(workspace_id).into_response()
                },
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}