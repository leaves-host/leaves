use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiToken {
    pub id: i64,
    pub contents: String,
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub id: i64,
    pub size: i32,
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub id: i64,
}
