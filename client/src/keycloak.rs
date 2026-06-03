use base64::engine::general_purpose;
use base64::Engine;
use chrono::NaiveDateTime;
use js_sys::Math;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use web_sys::{console, window};

use crate::config::AuthConfig;

const SCOPE: &str = "openid";

#[derive(Debug, Clone)]
pub enum AuthState {
    Loading(AuthConfig, OidcDiscovery, ProcessData),
    LoggedOut(AuthConfig, OidcDiscovery),
    LoggedIn(AuthConfig, OidcDiscovery, SessionData),
    Error(AuthConfig, OidcDiscovery, String),
    NotAvailable(),
}

#[derive(Debug, Clone, Deserialize)]
pub struct OidcDiscovery {
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: String,
    end_session_endpoint: String,
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    client_id: &'a str,
    code_verifier: &'a str,
    code: &'a str,
    redirect_uri: &'a str,
}

#[derive(Serialize)]
struct RefreshRequest<'a> {
    grant_type: &'a str,
    client_id: &'a str,
    refresh_token: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    expires_in: i64,
}

fn get_storage() -> Result<web_sys::Storage, String> {
    window()
        .ok_or_else(|| "No window object available".to_string())?
        .local_storage()
        .map_err(|_| "Could not access local storage".to_string())?
        .ok_or_else(|| "Local storage is not available".to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserData {
    pub sub: String,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: Option<String>,
    pub valid_until: NaiveDateTime,
    pub user: UserData,
}

impl SessionData {
    fn load() -> Option<Self> {
        let data = get_storage().ok()?.get_item("session_data").ok()??;
        serde_json::from_str(&data).ok()
    }

    fn save(&self) -> Result<(), String> {
        get_storage()?
            .set_item("session_data", &serde_json::to_string(self).unwrap())
            .map_err(|_| "Could not write session_data to local storage".to_string())
    }

    fn clear() -> Result<(), String> {
        get_storage()?
            .remove_item("session_data")
            .map_err(|_| "Could not remove session_data from local storage".to_string())
    }

    pub fn is_valid_for(&self, margin_secs: i64) -> bool {
        let threshold = chrono::Local::now().naive_local() + chrono::Duration::seconds(margin_secs);
        self.valid_until > threshold
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessData {
    pub code_verifier: String,
    pub state: String,
}

impl ProcessData {
    fn load() -> Option<Self> {
        let data = get_storage().ok()?.get_item("process_data").ok()??;
        serde_json::from_str(&data).ok()
    }

    fn save(&self) -> Result<(), String> {
        get_storage()?
            .set_item("process_data", &serde_json::to_string(self).unwrap())
            .map_err(|_| "Could not write process_data to local storage".to_string())
    }

    fn clear() -> Result<(), String> {
        get_storage()?
            .remove_item("process_data")
            .map_err(|_| "Could not remove process_data from local storage".to_string())
    }
}

impl AuthState {
    pub async fn new(config: AuthConfig) -> Self {
        let oidc_discovery = match fetch_discovery(&config).await {
            Ok(oidc_discovery) => oidc_discovery,
            Err(e) => {
                console::error_1(&format!("Error fetching OIDC discovery: {}", e).into());
                return AuthState::NotAvailable();
            }
        };

        if let Some(session) = SessionData::load() {
            let auth_state = AuthState::LoggedIn(config, oidc_discovery, session);
            return auth_state.refresh().await;
        }

        if let Some(process_data) = ProcessData::load() {
            return AuthState::Loading(config, oidc_discovery, process_data);
        }

        AuthState::LoggedOut(config, oidc_discovery)
    }

    pub async fn login(self, state: String) -> (Self, Option<String>) {
        console::debug_1(&"Starting Keycloak login process...".into());

        let (config, oidc_discovery) = match self {
            AuthState::Loading(config, od, _) => (config, od),
            AuthState::LoggedOut(config, od) => (config, od),
            AuthState::LoggedIn(_, _, _) => {
                console::info_1(&"Already logged in".into());
                return (self, None);
            }
            AuthState::Error(config, od, _) => (config, od),
            AuthState::NotAvailable() => {
                console::warn_1(&"AuthState is not suitable for login".into());
                return (self, None);
            }
        };

        let code_verifier = generate_random_string(128);
        let code_challenge = generate_code_challenge(&code_verifier);

        let process_data = ProcessData {
            code_verifier,
            state: state.clone(),
        };

        if let Err(e) = process_data.save() {
            console::error_1(&format!("Error saving process data: {}", e).into());
            return (
                AuthState::Error(
                    config,
                    oidc_discovery,
                    "Error saving process data".to_string(),
                ),
                None,
            );
        }

        let auth_url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
            oidc_discovery.authorization_endpoint,
            urlencoding::encode(&config.client_id),
            urlencoding::encode(&format!("{}/callback", &config.redirect)),
            urlencoding::encode(SCOPE),
            urlencoding::encode(&state),
            code_challenge,
        );

        console::debug_1(&format!("Keycloak auth URL: {}", auth_url).into());
        (
            AuthState::Loading(config, oidc_discovery, process_data),
            Some(auth_url),
        )
    }

    pub async fn callback(self, code: &str, state: &str) -> Self {
        console::debug_1(&format!("Handling callback – code: {}, state: {}", code, state).into());

        let (config, discovery, process_data) = match self {
            AuthState::Loading(config, discovery, p) => (config, discovery, p),
            invalid => {
                console::warn_1(
                    &format!("Received callback in invalid state: {:?}", invalid).into(),
                );
                return invalid;
            }
        };

        let token_response =
            match exchange_code_for_token(&config, &discovery, &process_data, code, state).await {
                Ok(r) => r,
                Err(e) => {
                    let _ = ProcessData::clear();
                    return AuthState::Error(config, discovery, e);
                }
            };

        if let Err(e) = ProcessData::clear() {
            console::error_1(&format!("Error clearing process data: {}", e).into());
        }

        match get_user_info(&discovery, &token_response.access_token).await {
            Ok(user) => {
                let valid_until = chrono::Local::now().naive_local()
                    + chrono::Duration::seconds(token_response.expires_in);

                let session_data = SessionData {
                    access_token: token_response.access_token,
                    id_token: token_response.id_token,
                    refresh_token: token_response.refresh_token,
                    valid_until,
                    user,
                };
                if let Err(e) = session_data.save() {
                    console::error_1(&format!("Error saving session: {}", e).into());
                    return AuthState::Error(config, discovery, "Error saving session".to_string());
                }
                return AuthState::LoggedIn(config, discovery, session_data);
            }
            Err(e) => return AuthState::Error(config, discovery, e),
        }
    }

    /// Verlängert eine abgelaufene Session mittels Refresh-Token, ohne den User
    /// erneut zur Keycloak-Login-Seite zu schicken.
    ///
    /// # Typischer Aufruf
    /// ```rust
    /// // Im Component, bevor ein API-Call gemacht wird:
    /// if let AuthState::LoggedIn(s) = &*auth {
    ///     if !s.is_valid_for(60) {
    ///         *auth.write() = auth.read().refresh().await;
    ///     }
    /// }
    /// ```
    pub async fn refresh(self) -> Self {
        let (config, oidc_discovery, session_data) = match self {
            AuthState::LoggedIn(config, oidc_discovery, session_data) => {
                (config, oidc_discovery, session_data)
            }
            other => {
                console::warn_1(&format!("Cannot refresh when not logged in: {:?}", other).into());
                return other;
            }
        };

        if session_data.is_valid_for(300) {
            console::debug_1(&"Session is still valid, no need to refresh".into());
            return AuthState::LoggedIn(config, oidc_discovery, session_data);
        }

        let refresh_token = match &session_data.refresh_token {
            Some(rt) => rt.clone(),
            None => {
                console::error_1(&"No refresh token available – full re-login required".into());
                let _ = SessionData::clear();
                return AuthState::LoggedOut(config, oidc_discovery);
            }
        };

        let discovery = match fetch_discovery(&config).await {
            Ok(d) => d,
            Err(e) => return AuthState::Error(config, oidc_discovery, e),
        };

        let refresh_request = RefreshRequest {
            grant_type: "refresh_token",
            client_id: &config.client_id,
            refresh_token: &refresh_token,
        };

        let client = reqwest::Client::new();
        let response = match client
            .post(&discovery.token_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&refresh_request)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return AuthState::Error(
                    config,
                    oidc_discovery,
                    format!("Refresh request failed: {}", e),
                )
            }
        };

        if !response.status().is_success() {
            // Refresh Token ist abgelaufen oder widerrufen → User muss sich neu anmelden
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            console::error_1(&format!("Refresh token rejected ({}): {}", status, body).into());
            let _ = SessionData::clear();
            return AuthState::LoggedOut(config, oidc_discovery);
        }

        let token_response: TokenResponse = match response.json().await {
            Ok(t) => t,
            Err(e) => {
                return AuthState::Error(
                    config,
                    oidc_discovery,
                    format!("Error parsing refresh response: {}", e),
                )
            }
        };

        let valid_until = chrono::Local::now().naive_local()
            + chrono::Duration::seconds(token_response.expires_in);

        let user = match get_user_info(&discovery, &token_response.access_token).await {
            Ok(user) => user,
            Err(e) => return AuthState::Error(config, discovery, e),
        };

        let new_session = SessionData {
            access_token: token_response.access_token,
            id_token: token_response.id_token,
            refresh_token: token_response.refresh_token,
            valid_until,
            user,
        };

        if let Err(e) = new_session.save() {
            let _ = ProcessData::clear();
            return AuthState::Error(
                config,
                oidc_discovery,
                format!("Error saving refreshed session: {}", e),
            );
        }

        console::debug_1(&"Session successfully refreshed".into());
        AuthState::LoggedIn(config, oidc_discovery, new_session)
    }

    pub async fn logout(self) -> (Self, Option<String>) {
        console::debug_1(&"Starting Keycloak logout process...".into());

        let (config, oidc_discovery, session_data) = match self {
            AuthState::LoggedIn(config, oidc_discovery, session_data) => {
                (config, oidc_discovery, session_data)
            }
            auth_state => {
                console::warn_1(&"Cannot logout when not logged in".into());
                return (auth_state, None);
            }
        };

        let logout_url = format!(
            "{}?client_id={}&id_token_hint={}&post_logout_redirect_uri={}",
            oidc_discovery.end_session_endpoint,
            urlencoding::encode(&config.client_id),
            urlencoding::encode(&session_data.id_token),
            urlencoding::encode(&config.redirect),
        );

        if let Err(e) = SessionData::clear() {
            console::warn_1(&format!("Error clearing session data during logout: {}", e).into());
        }

        console::debug_1(&format!("Keycloak logout URL: {}", logout_url).into());

        (
            AuthState::LoggedOut(config, oidc_discovery),
            Some(logout_url),
        )
    }
}

async fn get_user_info(discovery: &OidcDiscovery, access_token: &str) -> Result<UserData, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&discovery.userinfo_endpoint)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("UserInfo request error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("UserInfo request failed ({}): {}", status, body));
    }

    response
        .json::<UserData>()
        .await
        .map_err(|e| format!("Error parsing UserInfo response: {}", e))
}

async fn exchange_code_for_token(
    config: &AuthConfig,
    discovery: &OidcDiscovery,
    process_data: &ProcessData,
    code: &str,
    state: &str,
) -> Result<TokenResponse, String> {
    if state != process_data.state {
        return Err("State mismatch – possible CSRF attack".to_string());
    }

    let token_request = TokenRequest {
        grant_type: "authorization_code",
        client_id: &config.client_id,
        code_verifier: &process_data.code_verifier,
        code,
        redirect_uri: &format!("{}/callback", &config.redirect),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&discovery.token_endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&token_request)
        .send()
        .await
        .map_err(|e| format!("Token request error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Token request failed ({}): {}", status, body));
    }

    response
        .json::<TokenResponse>()
        .await
        .map_err(|e| format!("Error parsing token response: {}", e))
}

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~";
    (0..length)
        .map(|_| {
            let idx = (Math::random() * CHARSET.len() as f64) as usize;
            CHARSET[idx.min(CHARSET.len() - 1)] as char
        })
        .collect()
}

fn generate_code_challenge(code_verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize())
}

async fn fetch_discovery(config: &AuthConfig) -> Result<OidcDiscovery, String> {
    let url = format!(
        "{}/realms/{}/.well-known/openid-configuration",
        &config.domain,
        urlencoding::encode(&config.realm),
    );

    console::debug_1(&format!("Fetching OIDC discovery from {}", url).into());

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("OIDC discovery request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "OIDC discovery request failed ({})",
            response.status()
        ));
    }

    let discovery: OidcDiscovery = response
        .json()
        .await
        .map_err(|e| format!("Error parsing OIDC discovery response: {}", e))?;

    Ok(discovery)
}
