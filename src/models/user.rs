use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "userName")]
    pub user_name: String,
}
