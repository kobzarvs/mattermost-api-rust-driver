use super::{ApiClient, ApiError};
use reqwest::Method;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserMeResponse {
    pub id: u64,
    pub username: String,
    pub email: String,
}

impl ApiClient {
    pub async fn get_users_me(&self) -> Result<UserMeResponse, ApiError> {
        self.send_request(Method::GET, "/users/me", None::<()>)
            .await
    }
}
