use std::sync::{Arc, Mutex};

use base64::engine::general_purpose;
use base64::Engine;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use web_sys::{console, window};

use crate::{
    storage::{AuthData, LocalStorage, ProcessData, SessionData, StorageR, StorageW, UserData},
    Route,
};

// Auth0 Konfiguration
const AUTH0_DOMAIN: &str = "https://beancode.eu.auth0.com";
const CLIENT_ID: &str = "";
const REDIRECT_URI: &str = "http://localhost:8080/cook-and-run/callback";
const RETURN_URI: &str = "http://localhost:8080/cook-and-run";
const SCOPE: &str = "openid";

#[derive(Debug, Clone)]
enum AuthState {
    Loading,
    LoggedOut,
    LoggedIn(UserData),
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
    id_token: Option<String>,
}

fn get_auth_data() -> Result<AuthData, String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let storage = storage.lock().expect("Expected storage lock");
    storage.select_auth_data()
}

fn insert_auth_data(auth_data: AuthData) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");
    storage.insert_auth_data(auth_data)
}

#[component]
pub fn Callback(code: String, state: String, error: String) -> Element {
    let mut auth_state = use_signal(|| AuthState::Loading);
    use_effect(move || {
        let code = code.clone();
        let state = state.clone();
        let error = error.clone();
        spawn(async move {
            if !code.is_empty() && !state.is_empty() {
                match AuthService::exchange_code_for_token(&code, &state).await {
                    Ok((access_token, id_token)) => {
                        match AuthService::get_user_info(&access_token).await {
                            Ok(user) => {
                                console::log_1(&format!("User: {:?}", user.sub).into());
                                auth_state.set(AuthState::LoggedIn(user.clone()));
                                insert_auth_data(AuthData {
                                    process_data: None,
                                    session_data: Some(SessionData {
                                        access_token,
                                        id_token,
                                        user,
                                    }),
                                })
                                .unwrap();
                                use_navigator().push(Route::Dashboard {});
                            }
                            Err(e) => auth_state.set(AuthState::Error(e)),
                        }
                    }
                    Err(e) => auth_state.set(AuthState::Error(e)),
                }
            } else if !error.is_empty() {
                auth_state.set(AuthState::Error(error));
            } else {
                use_navigator().push(Route::Dashboard {});
            }
        });
    });
    rsx! {
        div { class: "min-h-screen bg-gray-50 py-8",
            div { class: "max-w-md mx-auto bg-white rounded-lg shadow-md p-6",
                h1 { class: "text-2xl font-bold text-center mb-6", "Dioxus Auth0 Demo" }
                match auth_state() {
                    AuthState::Loading => rsx! {
                        div { class: "text-center",
                            div { class: "animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500 mx-auto mb-2" }
                            p { "Loading..." }
                        }
                    },
                    AuthState::LoggedOut => rsx! {
                        div { class: "text-center",
                            p { class: "mb-4 text-gray-600", "You are not logged in." }
                        }
                    },
                    AuthState::LoggedIn(_) => rsx! {
                        div { class: "text-center",
                            p { class: "mb-4 text-gray-600", "You are logged in." }
                        }
                    },
                    AuthState::Error(error) => rsx! {
                        div { class: "text-center",
                            div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                                strong { "Error: " }
                                span { "{error}" }
                            }
                        }
                    },
                }
            }
        }
    }
}

pub struct AuthService;

impl AuthService {
    pub fn login() {
        let code_verifier = generate_random_string(128);
        let code_challenge = generate_code_challenge(&code_verifier);
        let state = generate_random_string(32);

        insert_auth_data(AuthData {
            session_data: None,
            process_data: Some(ProcessData {
                code_verifier: code_verifier.clone(),
                state: state.clone(),
            }),
        })
        .unwrap();

        let auth_url = format!(
            "{}/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
            AUTH0_DOMAIN,
            CLIENT_ID,
            urlencoding::encode(REDIRECT_URI),
            urlencoding::encode(SCOPE),
            state,
            code_challenge
        );

        window().unwrap().location().set_href(&auth_url).unwrap();
    }

    pub fn logout() {
        insert_auth_data(AuthData {
            session_data: None,
            process_data: None,
        })
        .unwrap();

        let logout_url = format!(
            "{}/v2/logout?client_id={}&returnTo={}",
            AUTH0_DOMAIN,
            CLIENT_ID,
            urlencoding::encode(RETURN_URI)
        );

        window().unwrap().location().set_href(&logout_url).unwrap();
    }

    async fn exchange_code_for_token(code: &str, state: &str) -> Result<(String, String), String> {
        let auth_data = get_auth_data();
        if let Err(e) = auth_data {
            return Err(format!("Error while loading auth data: {}", e));
        }

        let auth_data = auth_data.expect("Expected auth data");

        if auth_data.process_data.is_none() {
            return Err("No process data found in auth data".to_string());
        }

        let process_data = auth_data.process_data.expect("Expected process data");

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
            token_response.id_token.unwrap_or_default(),
        ))
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
