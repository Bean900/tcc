use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub auth: AuthConfig,
    pub legal: LegalConfig,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct AuthConfig {
    pub domain: String,
    pub realm: String,
    pub client_id: String,
    pub redirect: String,
    pub audience: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct LegalConfig {
    pub operator_name: String,
    pub company_name: Option<String>,
    pub address_street: String,
    pub address_zip: String,
    pub address_city: String,
    pub address_country: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub vat_id: Option<String>,
    pub commercial_register_number: Option<String>,
    pub commercial_register_court: Option<String>,
    pub editorial_responsible: Option<String>,

    pub privacy_officer_name: Option<String>,
    pub privacy_officer_email: Option<String>,
    pub supervisory_authority: Option<String>,
    pub hosting_provider: String,
    pub server_location: String,
    pub data_retention_logs_days: u32,
    pub privacy_last_updated: Option<String>,

    pub cookie_consent_enabled: bool,
    pub consent_version: String,

    pub matomo_url: Option<String>,
    pub matomo_site_id: Option<u32>,
    pub plausible_domain: Option<String>,
    pub plausible_api_host: Option<String>,
    pub google_analytics_id: Option<String>,

    pub social_github: Option<String>,
    pub social_twitter: Option<String>,
    pub social_linkedin: Option<String>,
    pub social_instagram: Option<String>,

    pub app_name: String,
    pub copyright_year: Option<String>,
    pub impressum_url_override: Option<String>,
    pub privacy_url_override: Option<String>,
}

impl LegalConfig {
    pub async fn fetch() -> Result<Self, String> {
        let client = reqwest::Client::new();
        let response = client
            .get("/legal-config.json")
            .send()
            .await
            .map_err(|e| format!("Netzwerkfehler beim Laden der Legal-Config: {e}"))?;

        if !response.status().is_success() {
            return Err(format!(
                "legal-config.json nicht erreichbar (HTTP {}). \
                 Bitte public/legal-config.json anlegen.",
                response.status()
            ));
        }

        response
            .json::<Self>()
            .await
            .map_err(|e| format!("Fehler beim Parsen der Legal-Config: {e}"))
    }

    pub fn has_analytics(&self) -> bool {
        self.matomo_url.is_some()
            || self.plausible_domain.is_some()
            || self.google_analytics_id.is_some()
    }

    pub fn has_social(&self) -> bool {
        self.social_github.is_some()
            || self.social_twitter.is_some()
            || self.social_linkedin.is_some()
            || self.social_instagram.is_some()
    }
}

impl Config {
    pub async fn fetch() -> Result<Self, String> {
        let window = web_sys::window().expect("no global `window` exists");
        let location = window.location();
        let base_url = location.origin().unwrap_or_else(|_| "unknown".to_string());
        let client = reqwest::Client::new();
        let path = format!("{}/config.json", base_url);

        let res = client.get(path).send().await;

        match res {
            Ok(response) if response.status().is_success() => {
                response.json::<Config>().await.map_err(|e| e.to_string())
            }
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }
}
