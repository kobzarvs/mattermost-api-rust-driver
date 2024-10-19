#![allow(dead_code)]

pub mod users;
pub mod auth;

use reqwest::Method;
use serde::Deserialize;
use thiserror::Error;

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

// Обновленная структура ошибки API
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Server returned error: {status_code} - {message}")]
    ServerError {
        id: String,
        message: String,
        detailed_error: String,
        request_id: String,
        status_code: u16,
    },

    #[error("Unsupported HTTP method")]
    UnsupportedMethod,
}

// Структура для десериализации ответа сервера при ошибке
#[derive(Deserialize)]
pub struct ServerErrorResponse {
    id: String,
    message: String,
    detailed_error: String,
    request_id: String,
}

// ApiClient с универсальным запросом
impl ApiClient {
    pub fn new(base_url: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        println!("{}", base_url);
        Ok(ApiClient { client, base_url })
    }
    pub async fn send_request<T: for<'de> Deserialize<'de>>(
        &self,
        method: Method,
        path: &str,
        body: Option<impl serde::Serialize>, // Данные для POST/PUT
    ) -> Result<T, ApiError> {
        let url = self.get_api_route(path); // Генерация полного URL
        println!("{} {}", method.as_str(), url);

        let client = &self.client;

        // Отправляем запрос в зависимости от метода
        let request = match method {
            Method::GET => client.get(&url),
            Method::POST => {
                let mut req = client.post(&url);
                if let Some(b) = body {
                    req = req.json(&b);
                }
                req
            }

            Method::PUT => {
                let mut req = client.put(&url);
                if let Some(b) = body {
                    req = req.json(&b);
                }
                req
            }

            Method::DELETE => client.delete(&url),

            // Возвращаем кастомную ошибку
            _ => return Err(ApiError::UnsupportedMethod),
        };

        // Отправляем запрос и обрабатываем результат
        let response = request.send().await?;

        self.handle_response(response).await
    }

    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, ApiError> {
        let status = response.status();

        if status.is_success() {
            Ok(response.json::<T>().await?)
        } else {
            let error_response = response.json::<ServerErrorResponse>().await?;
            Err(ApiError::ServerError {
                id: error_response.id,
                message: error_response.message,
                detailed_error: error_response.detailed_error,
                request_id: error_response.request_id,
                status_code: status.into(),
            })
        }
    }

    pub fn get_route(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub fn get_api_route(&self, path: &str) -> String {
        format!("{}/api/v4{}", self.base_url, path)
    }
}
