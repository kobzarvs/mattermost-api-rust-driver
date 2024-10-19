use std::sync::Arc;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::api::ApiClient;

mod api;

#[tokio::main]
async fn main() {
    let api_client = Arc::clone(&API_CLIENT);

    let get_users_me = tokio::spawn(async move {
        let api = api_client.lock().await;
        match api.get_users_me().await {
            Ok(user) => {
                // Обработка успешного результата
                println!("User ID: {}", user.id);
                println!("Username: {}", user.username);
                println!("Email: {}", user.email);
            }
            Err(e) => {
                // Обработка ошибки
                eprintln!("{}", e);
            }
        }
    });

    get_users_me.await.expect("Failed to get_users_me method")
}

// Глобальный синглтон ApiClient
pub static API_CLIENT: Lazy<Arc<Mutex<ApiClient>>> = Lazy::new(|| {
    let base_url = "http://localhost:8065";
    let client = ApiClient::new(base_url.into()).expect("Failed to create API Client");
    Arc::new(Mutex::new(client))
});
