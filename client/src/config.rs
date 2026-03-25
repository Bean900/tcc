use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct AppConfig {
    pub auth0_domain: String,
    pub auth0_client_id: String,
    pub auth0_redirect: String,
    pub auth0_audience: String,
}

impl AppConfig {
    pub async fn fetch() -> Result<Self, String> {
        let window = web_sys::window().expect("no global `window` exists");
        let location = window.location();
        let base_url = location.origin().unwrap_or_else(|_| "unknown".to_string());
        let client = reqwest::Client::new();
        let path = format!("{}/config.json", base_url);
        let res = client.get(path).send().await;

        match res {
            Ok(response) if response.status().is_success() => response
                .json::<AppConfig>()
                .await
                .map_err(|e| e.to_string()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }
}
