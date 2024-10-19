use super::ApiClient;

impl ApiClient {
    // Пример функции API для выполнения GET-запроса
    pub async fn login(&self) -> Result<String, reqwest::Error> {
        // let url = "https://band.wb.ru/api/v4/users/me";
        let url = "https://mattermost-stage.wb.ru/oauth/gitlab/login";
        println!("GET {}", url);

        let response = self.client.get(url).send().await?;

        println!("Response: {:?}", response);
        let body = response.text().await?;

        Ok(body)
    }
}
