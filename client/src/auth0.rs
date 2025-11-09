use base64::engine::general_purpose;
use base64::Engine;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use web_sys::{console, window};

const AUTH0_DOMAIN: &str = "https://beancode.eu.auth0.com";
const CLIENT_ID: &str = "KPdjRob3k5SRCqs4wExmQOPrOkqaUJTQ";
const REDIRECT_URI: &str = "http://localhost:8080";
const AUDIENCE: &str = "https://home.beancode.de/tcc/backend";
const SCOPE: &str = "read:cook_and_run delete:cook_and_run update:cook_and_run openid";

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

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserData {
    pub sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    pub access_token: String,
    pub valid_until: NaiveDateTime,
    pub user: UserData,
}

impl SessionData {
    fn load() -> Option<Self> {
        let storage = window()
            .map_or_else(|| None, |w| Some(w.local_storage().ok()?))
            .flatten()?;

        let data = storage.get_item("session_data").ok()??;

        serde_json::from_str(&data).ok()
    }

    fn save(&self) -> Result<(), String> {
        let storage = window()
            .map_or_else(|| None, |w| Some(w.local_storage().ok()?))
            .flatten()
            .ok_or_else(|| "Could not access local storage".to_string())?;

        let _ = storage.set_item(
            "session_data",
            serde_json::to_string(self).unwrap().as_str(),
        );
        Ok(())
    }

    fn clear() -> Result<(), String> {
        let storage = window()
            .map_or_else(|| None, |w| Some(w.local_storage().ok()?))
            .flatten()
            .ok_or_else(|| "Could not access local storage".to_string())?;

        let _ = storage.remove_item("session_data");
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessData {
    pub code_verifier: String,
    pub state: String,
}

impl ProcessData {
    fn load() -> Option<Self> {
        let storage = window()
            .map_or_else(|| None, |w| Some(w.local_storage().ok()?))
            .flatten()?;

        let data = storage.get_item("process_data").ok()??;

        serde_json::from_str(&data).ok()
    }

    fn save(&self) -> Result<(), String> {
        let storage = window()
            .map_or_else(|| None, |w| Some(w.local_storage().ok()?))
            .flatten()
            .ok_or_else(|| "Could not access local storage".to_string())?;

        let _ = storage.set_item(
            "process_data",
            serde_json::to_string(self).unwrap().as_str(),
        );
        Ok(())
    }

    fn clear() -> Result<(), String> {
        let storage = window()
            .map_or_else(|| None, |w| Some(w.local_storage().ok()?))
            .flatten()
            .ok_or_else(|| "Could not access local storage".to_string())?;

        let _ = storage.remove_item("process_data");
        Ok(())
    }
}

impl AuthState {
    pub fn new() -> Self {
        if let Some(session) = SessionData::load() {
            if session.valid_until > chrono::Local::now().naive_local() {
                return AuthState::LoggedIn(session);
            }
        }

        if let Some(process_data) = ProcessData::load() {
            return AuthState::Loading(process_data);
        }
        AuthState::LoggedOut
    }

    pub fn login(redirect_uri: &str) -> (Self, String) {
        let code_verifier = generate_random_string(128);
        let code_challenge = generate_code_challenge(&code_verifier);
        let state = generate_random_string(32);

        let process_data = ProcessData {
            code_verifier: code_verifier.clone(),
            state: state.clone(),
        };
        let result = process_data.save();
        if let Err(r) = result {
            console::error_1(&format!("Error saving process data: {}", r).into());
            return (
                AuthState::Error("Error saving process data".to_string()),
                "".to_string(),
            );
        }
        let auth_url = format!(
            "{}/authorize?response_type=code&client_id={}&redirect_uri={}{}&scope={}&audience={}&state={}&code_challenge={}&code_challenge_method=S256",
            AUTH0_DOMAIN,
            CLIENT_ID,
            urlencoding::encode(REDIRECT_URI),
            urlencoding::encode(redirect_uri),
            urlencoding::encode(SCOPE),
            urlencoding::encode(AUDIENCE),
            state,
            code_challenge
        );
        (AuthState::Loading(process_data), auth_url)
    }

    pub async fn callback(&self, code: String, state: String) -> Self {
        let process_data = match self {
            AuthState::Loading(data) => data,
            _ => return AuthState::Error("Invalid auth state for callback".to_string()),
        };

        let (access_token, valid_until) =
            match exchange_code_for_token(process_data, &code, &state).await {
                Ok((access_token, valid_until)) => (access_token, valid_until),
                Err(e) => return AuthState::Error(e),
            };

        let _ = ProcessData::clear();
        match get_user_info(&access_token).await {
            Ok(user) => {
                let session_data = SessionData {
                    access_token,
                    valid_until,
                    user,
                };
                let _ = session_data.save();
                AuthState::LoggedIn(session_data)
            }
            Err(e) => AuthState::Error(e),
        }
    }

    pub fn logout(&self, return_to_path: &str) -> (Self, String) {
        if !matches!(self, AuthState::LoggedIn(_)) {
            console::error_1(&"Cannot logout when not logged in".into());
            return (self.clone(), "".to_string());
        }

        let logout_url = format!(
            "{}/v2/logout?client_id={}&returnTo={}{}",
            AUTH0_DOMAIN,
            CLIENT_ID,
            urlencoding::encode(REDIRECT_URI),
            urlencoding::encode(return_to_path)
        );
        let _ = SessionData::clear();
        (AuthState::LoggedOut, logout_url)
    }
}

async fn get_user_info(access_token: &str) -> Result<UserData, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/userinfo", AUTH0_DOMAIN))
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("User info request error: {}", e))?;

    if !response.status().is_success() {
        return Err("User info request was not successfull".to_string());
    }

    let user: UserData = response
        .json()
        .await
        .map_err(|e| format!("Fehler beim Parsen der UserInfo: {}", e))?;

    Ok(user)
}

async fn exchange_code_for_token(
    process_data: &ProcessData,
    code: &str,
    state: &str,
) -> Result<(String, NaiveDateTime), String> {
    if state != process_data.state {
        return Err("State doesn't match".to_string());
    }

    let token_request = TokenRequest {
        grant_type: "authorization_code",
        client_id: CLIENT_ID,
        code_verifier: &process_data.code_verifier,
        code,
        redirect_uri: REDIRECT_URI,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/oauth/token", AUTH0_DOMAIN))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&token_request)
        .send()
        .await
        .map_err(|e| format!("Token request error: {}", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Token request error: {}", error_text));
    }

    let token_response: TokenResponse = response
        .json()
        .await
        .map_err(|e| format!("Error while parsing token response: {}", e))?;

    Ok((
        token_response.access_token,
        chrono::Local::now().naive_local() + chrono::Duration::seconds(token_response.expires_in),
    ))
}

// PKCE Helper Funktionen
fn generate_random_string(length: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    charset[..length.min(charset.len())].to_string()
}

fn generate_code_challenge(code_verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let result = hasher.finalize();
    general_purpose::URL_SAFE_NO_PAD.encode(&result)
}
