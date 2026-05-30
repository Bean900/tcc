use base64::engine::general_purpose;
use base64::Engine;
use chrono::NaiveDateTime;
use js_sys::Math;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use web_sys::{console, window};

use crate::config::AppConfig;

const SCOPE: &str = "openid";

#[derive(Debug, Clone, Deserialize)]
struct OidcDiscovery {
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: String,
    end_session_endpoint: String,
}

thread_local! {
    static OIDC_CACHE: RefCell<Option<OidcDiscovery>> = const { RefCell::new(None) };
}

async fn fetch_discovery(config: &AppConfig) -> Result<OidcDiscovery, String> {
    let cached = OIDC_CACHE.with(|c| c.borrow().clone());
    if let Some(discovery) = cached {
        return Ok(discovery);
    }

    let url = format!(
        "{}/realms/{}/.well-known/openid-configuration",
        &config.auth_domain,
        urlencoding::encode(&config.auth_realm),
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

    // Im Cache ablegen
    OIDC_CACHE.with(|c| *c.borrow_mut() = Some(discovery.clone()));

    Ok(discovery)
}

#[derive(Debug, Clone)]
pub enum AuthState {
    Loading(ProcessData),
    LoggedOut,
    LoggedIn(SessionData),
    Error(String),
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
    pub fn new() -> Self {
        if let Some(session) = SessionData::load() {
            if session.is_valid_for(0) {
                return AuthState::LoggedIn(session);
            }
            // Abgelaufene Session bereinigen; Caller kann danach `refresh()` aufrufen
            // wenn ein refresh_token gespeichert war.
            let _ = SessionData::clear();
        }

        if let Some(process_data) = ProcessData::load() {
            return AuthState::Loading(process_data);
        }

        AuthState::LoggedOut
    }

    pub async fn login(config: &AppConfig, state: String) -> (Self, String) {
        console::debug_1(&"Starting Keycloak login process...".into());

        let discovery = match fetch_discovery(config).await {
            Ok(d) => d,
            Err(e) => {
                console::error_1(&format!("OIDC discovery failed: {}", e).into());
                return (AuthState::Error(e), String::new());
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
                AuthState::Error("Error saving process data".to_string()),
                String::new(),
            );
        }

        let auth_url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
            discovery.authorization_endpoint,
            urlencoding::encode(&config.auth_client_id),
            urlencoding::encode(&format!("{}/callback", &config.auth_redirect)),
            urlencoding::encode(SCOPE),
            urlencoding::encode(&state),
            code_challenge,
        );

        console::debug_1(&format!("Keycloak auth URL: {}", auth_url).into());
        (AuthState::Loading(process_data), auth_url)
    }

    pub async fn callback(&self, config: &AppConfig, code: &str, state: &str) -> Self {
        console::debug_1(&format!("Handling callback – code: {}, state: {}", code, state).into());

        let process_data = match self {
            AuthState::Loading(data) => data,
            _ => {
                let _ = ProcessData::clear();
                return AuthState::Error("Invalid auth state for callback".to_string());
            }
        };

        let discovery = match fetch_discovery(config).await {
            Ok(d) => d,
            Err(e) => {
                let _ = ProcessData::clear();
                return AuthState::Error(e);
            }
        };

        let token_response =
            match exchange_code_for_token(config, &discovery, process_data, code, state).await {
                Ok(r) => r,
                Err(e) => {
                    let _ = ProcessData::clear();
                    return AuthState::Error(e);
                }
            };

        let _ = ProcessData::clear();

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
                    return AuthState::Error("Error saving session".to_string());
                }
                AuthState::LoggedIn(session_data)
            }
            Err(e) => AuthState::Error(e),
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
    ///         *auth.write() = auth.read().refresh(&config).await;
    ///     }
    /// }
    /// ```
    pub async fn refresh(&self, config: &AppConfig) -> Self {
        let session = match self {
            AuthState::LoggedIn(s) => s,
            other => return other.clone(),
        };

        let refresh_token = match &session.refresh_token {
            Some(rt) => rt.clone(),
            None => {
                console::error_1(&"No refresh token available – full re-login required".into());
                let _ = SessionData::clear();
                return AuthState::LoggedOut;
            }
        };

        let discovery = match fetch_discovery(config).await {
            Ok(d) => d,
            Err(e) => return AuthState::Error(e),
        };

        let refresh_request = RefreshRequest {
            grant_type: "refresh_token",
            client_id: &config.auth_client_id,
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
            Err(e) => return AuthState::Error(format!("Refresh request failed: {}", e)),
        };

        if !response.status().is_success() {
            // Refresh Token ist abgelaufen oder widerrufen → User muss sich neu anmelden
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            console::error_1(&format!("Refresh token rejected ({}): {}", status, body).into());
            let _ = SessionData::clear();
            return AuthState::LoggedOut;
        }

        let token_response: TokenResponse = match response.json().await {
            Ok(t) => t,
            Err(e) => return AuthState::Error(format!("Error parsing refresh response: {}", e)),
        };

        let valid_until = chrono::Local::now().naive_local()
            + chrono::Duration::seconds(token_response.expires_in);

        let new_session = SessionData {
            access_token: token_response.access_token,
            id_token: token_response.id_token,
            refresh_token: token_response
                .refresh_token
                .or(session.refresh_token.clone()),
            valid_until,
            user: session.user.clone(),
        };

        if let Err(e) = new_session.save() {
            return AuthState::Error(format!("Error saving refreshed session: {}", e));
        }

        console::debug_1(&"Session successfully refreshed".into());
        AuthState::LoggedIn(new_session)
    }

    pub async fn logout(&self, config: &AppConfig) -> (Self, String) {
        console::debug_1(&"Starting Keycloak logout process...".into());

        let session = match self {
            AuthState::LoggedIn(s) => s,
            _ => {
                console::error_1(&"Cannot logout when not logged in".into());
                return (self.clone(), String::new());
            }
        };

        let discovery = match fetch_discovery(config).await {
            Ok(d) => d,
            Err(e) => return (AuthState::Error(e), String::new()),
        };

        let logout_url = format!(
            "{}?client_id={}&id_token_hint={}&post_logout_redirect_uri={}",
            discovery.end_session_endpoint,
            urlencoding::encode(&config.auth_client_id),
            urlencoding::encode(&session.id_token),
            urlencoding::encode(&config.auth_redirect),
        );

        let _ = SessionData::clear();
        console::debug_1(&format!("Keycloak logout URL: {}", logout_url).into());
        (AuthState::LoggedOut, logout_url)
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
    config: &AppConfig,
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
        client_id: &config.auth_client_id,
        code_verifier: &process_data.code_verifier,
        code,
        redirect_uri: &format!("{}/callback", &config.auth_redirect),
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
