use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiToken {
    pub contents: String,
    pub id: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileInfo {
    pub id: String,
    pub size: u64,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Signup {
    pub email: String,
    pub id: u64,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Upload {
    pub id: String,
    pub size: u64,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub id: i64,
}
