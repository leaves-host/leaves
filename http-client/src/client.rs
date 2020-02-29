use crate::{
    config::Config,
    error::{Error, Result},
};
use isahc::{
    http::{Method, Request, StatusCode},
    Body, HttpClient, ResponseExt,
};
use models::v1::*;
use serde::de::DeserializeOwned;
use serde_json::json;

#[derive(Debug)]
#[non_exhaustive]
pub struct Client {
    pub config: Config,
    http: HttpClient,
}

impl Client {
    pub fn new(config: Config) -> Result<Self> {
        let http = HttpClient::new().map_err(|why| Error::BuildingHttpClient { source: why })?;

        Ok(Self { config, http })
    }

    pub fn file_info(&self, id: impl AsRef<str>) -> Result<FileInfo> {
        self._file_info(id.as_ref())
    }

    fn _file_info(&self, id: &str) -> Result<FileInfo> {
        let mut path = String::with_capacity(6 + id.len());
        path.push_str("files/");
        path.push_str(id);

        self.get(&path, false)
    }

    pub fn me(&self) -> Result<User> {
        self.get("users/@me", true)
    }

    pub fn signup(&self, email: impl AsRef<str>) -> Result<Signup> {
        self._signup(email.as_ref())
    }

    fn _signup(&self, email: &str) -> Result<Signup> {
        let body = json!({
            "email": email,
        })
        .to_string();

        self.post("users", false, body.into_bytes())
    }

    pub fn upload(&self, body: Vec<u8>) -> Result<Upload> {
        self.post("files", true, body)
    }

    pub fn user_api_tokens(&self) -> Result<Vec<ApiToken>> {
        self.get("users/@me/api-tokens", false)
    }

    fn get<T: DeserializeOwned>(&self, path: &str, auth: bool) -> Result<T> {
        self.send(path, auth, ().into())
    }

    fn post<T: DeserializeOwned>(&self, path: &str, auth: bool, body: Vec<u8>) -> Result<T> {
        self.send(path, auth, body.into())
    }

    fn send<T: DeserializeOwned>(&self, path: &str, auth: bool, body: Body) -> Result<T> {
        let mut builder = Request::builder();
        builder.uri(format!("{}/{}", self.config.api_url, path));
        builder.method(Method::POST);

        if auth {
            if let Some(auth) = self.config.auth() {
                builder.header("Authorization", auth);
            }
        }

        let req = builder
            .body(body)
            .map_err(|source| Error::BuildingRequest { source })?;
        let mut res = self
            .http
            .send(req)
            .map_err(|why| Error::SendingRequest { source: why })?;

        match res.status() {
            StatusCode::NOT_FOUND => return Err(Error::NotFound),
            StatusCode::UNAUTHORIZED => return Err(Error::Unauthorized),
            StatusCode::INTERNAL_SERVER_ERROR => return Err(Error::InternalServerError),
            _ => {}
        }

        let mut contents = Vec::new();
        res.copy_to(&mut contents)
            .map_err(|source| Error::CopyingResponseBody { source })?;

        serde_json::from_slice(&contents)
            .map_err(|source| Error::DeserializingBody { contents, source })
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use std::fmt::Debug;

    static_assertions::assert_impl_all!(Client: Debug, Send, Sync);
}
