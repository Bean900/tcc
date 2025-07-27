use base64::engine::general_purpose;
use base64::Engine;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use web_sys::{console, window};

// Auth0 Konfiguration
const AUTH0_DOMAIN: &str = "https://beancode.eu.auth0.com";
const CLIENT_ID: &str = "KPdjRob3k5SRCqs4wExmQOPrOkqaUJTQ";
const REDIRECT_URI: &str = "http://localhost:8080/callback";
const SCOPE: &str = "openid";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct User {
    sub: String,
}

#[derive(Debug, Clone)]
enum AuthState {
    Loading,
    LoggedOut,
    LoggedIn(User),
    Error(String),
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

// URL Query Parameter Parser
fn parse_query_params(query: &str) -> HashMap<String, String> {
    query
        .trim_start_matches('?')
        .split('&')
        .filter_map(|param| {
            let mut parts = param.split('=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((
                    urlencoding::decode(key).ok()?.to_string(),
                    urlencoding::decode(value).ok()?.to_string(),
                )),
                _ => None,
            }
        })
        .collect()
}

// Auth Service
struct AuthService;

impl AuthService {
    fn login() {
        let code_verifier = generate_random_string(128);
        let code_challenge = generate_code_challenge(&code_verifier);
        let state = generate_random_string(32);

        // PKCE Parameter im localStorage speichern
        if let Some(storage) = window().unwrap().local_storage().unwrap() {
            storage
                .set_item("auth0_code_verifier", &code_verifier)
                .unwrap();
            storage.set_item("auth0_state", &state).unwrap();
        }

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

    fn logout() {
        // Token aus localStorage entfernen
        if let Some(storage) = window().unwrap().local_storage().unwrap() {
            storage.remove_item("auth0_access_token").unwrap();
            storage.remove_item("auth0_id_token").unwrap();
            storage.remove_item("auth0_user").unwrap();
        }

        let logout_url = format!(
            "{}/v2/logout?client_id={}&returnTo={}",
            AUTH0_DOMAIN,
            CLIENT_ID,
            urlencoding::encode("http://localhost:8080")
        );

        window().unwrap().location().set_href(&logout_url).unwrap();
    }

    async fn exchange_code_for_token(code: &str, state: &str) -> Result<(String, String), String> {
        let storage = window()
            .unwrap()
            .local_storage()
            .unwrap()
            .ok_or("localStorage nicht verfügbar")?;

        let stored_state = storage
            .get_item("auth0_state")
            .map_err(|_| "Fehler beim Lesen des gespeicherten State")?
            .ok_or("Kein State im localStorage gefunden")?;

        if state != stored_state {
            return Err("State Parameter stimmt nicht überein".to_string());
        }

        let code_verifier = storage
            .get_item("auth0_code_verifier")
            .map_err(|_| "Fehler beim Lesen des Code Verifiers")?
            .ok_or("Kein Code Verifier im localStorage gefunden")?;

        // Cleanup
        storage.remove_item("auth0_state").unwrap();
        storage.remove_item("auth0_code_verifier").unwrap();

        let token_request = TokenRequest {
            grant_type: "authorization_code",
            client_id: CLIENT_ID,
            code_verifier: &code_verifier,
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
            .map_err(|e| format!("Token Request Fehler: {}", e))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unbekannter Fehler".to_string());
            return Err(format!("Token Request fehlgeschlagen: {}", error_text));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| format!("Fehler beim Parsen der Token Response: {}", e))?;

        // Token speichern
        storage
            .set_item("auth0_access_token", &token_response.access_token)
            .unwrap();

        if let Some(id_token) = &token_response.id_token {
            storage.set_item("auth0_id_token", id_token).unwrap();
        }

        Ok((
            token_response.access_token,
            token_response.id_token.unwrap_or_default(),
        ))
    }

    async fn get_user_info(access_token: &str) -> Result<User, String> {
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/userinfo", AUTH0_DOMAIN))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| format!("UserInfo Request Fehler: {}", e))?;

        if !response.status().is_success() {
            return Err("UserInfo Request fehlgeschlagen".to_string());
        }

        let user: User = response
            .json()
            .await
            .map_err(|e| format!("Fehler beim Parsen der UserInfo: {}", e))?;

        // User im localStorage speichern
        if let Some(storage) = window().unwrap().local_storage().unwrap() {
            let user_json = serde_json::to_string(&user).unwrap();
            storage.set_item("auth0_user", &user_json).unwrap();
        }

        Ok(user)
    }

    fn get_stored_user() -> Option<User> {
        let storage = window().unwrap().local_storage().unwrap()?;
        let user_json = storage.get_item("auth0_user").ok()??;
        serde_json::from_str(&user_json).ok()
    }

    fn has_valid_token() -> bool {
        if let Some(storage) = window().unwrap().local_storage().unwrap() {
            storage.get_item("auth0_access_token").unwrap().is_some()
        } else {
            false
        }
    }
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
    token_type: String,
    expires_in: u64,
}

// Hook für Auth State Management
fn use_auth() -> Signal<AuthState> {
    let mut auth_state = use_signal(|| AuthState::Loading);

    use_effect(move || {
        spawn(async move {
            // Prüfen ob wir von einem Callback kommen
            let location = window().unwrap().location();
            let pathname = location.pathname().unwrap();
            let search = location.search().unwrap();

            if pathname == "/callback" && !search.is_empty() {
                let params = parse_query_params(&search);

                if let (Some(code), Some(state)) = (params.get("code"), params.get("state")) {
                    match AuthService::exchange_code_for_token(code, state).await {
                        Ok((access_token, _)) => {
                            match AuthService::get_user_info(&access_token).await {
                                Ok(user) => {
                                    console::log_1(&format!("User: {:?}", user.sub).into());
                                    auth_state.set(AuthState::LoggedIn(user));
                                    // Redirect zur Hauptseite
                                    window().unwrap().location().set_href("/").unwrap();
                                }
                                Err(e) => auth_state.set(AuthState::Error(e)),
                            }
                        }
                        Err(e) => auth_state.set(AuthState::Error(e)),
                    }
                } else if let Some(error) = params.get("error") {
                    let error_description = params
                        .get("error_description")
                        .cloned()
                        .unwrap_or_else(|| error.clone());
                    auth_state.set(AuthState::Error(error_description));
                }
            } else {
                // Prüfen ob bereits eingeloggt
                if AuthService::has_valid_token() {
                    if let Some(user) = AuthService::get_stored_user() {
                        auth_state.set(AuthState::LoggedIn(user));
                    } else {
                        auth_state.set(AuthState::LoggedOut);
                    }
                } else {
                    auth_state.set(AuthState::LoggedOut);
                }
            }
        });
    });

    auth_state
}

// Komponenten
#[component]
fn LoginButton() -> Element {
    rsx! {
        button {
            class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
            onclick: move |_| AuthService::login(),
            "Mit Auth0 anmelden"
        }
    }
}

#[component]
fn UserProfile(user: User) -> Element {
    rsx! {
        div { class: "flex items-center space-x-4 p-4 bg-gray-100 rounded",
            div {
                h3 { class: "font-semibold", "{user.sub}" }
                button {
                    class: "mt-2 px-3 py-1 bg-red-500 text-white text-sm rounded hover:bg-red-600",
                    onclick: move |_| AuthService::logout(),
                    "Abmelden"
                }
            }
        }
    }
}

#[component]
fn CallbackPage() -> Element {
    rsx! {
        div { class: "flex items-center justify-center min-h-screen",
            div { class: "text-center",
                div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto mb-4" }
                p { "Authentifizierung wird verarbeitet..." }
            }
        }
    }
}

// Hauptkomponente
#[component]
pub fn App() -> Element {
    let auth_state = use_auth();

    // Router-ähnliche Logik
    let location = window().unwrap().location();
    let pathname = location.pathname().unwrap();

    if pathname == "/callback" {
        return rsx! {
            CallbackPage {}
        };
    }

    rsx! {
        div { class: "min-h-screen bg-gray-50 py-8",
            div { class: "max-w-md mx-auto bg-white rounded-lg shadow-md p-6",
                h1 { class: "text-2xl font-bold text-center mb-6", "Dioxus Auth0 Demo" }
                match auth_state() {
                    AuthState::Loading => rsx! {
                        div { class: "text-center",
                            div { class: "animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500 mx-auto mb-2" }
                            p { "Lade..." }
                        }
                    },
                    AuthState::LoggedOut => rsx! {
                        div { class: "text-center",
                            p { class: "mb-4 text-gray-600", "Du bist nicht angemeldet." }
                            LoginButton {}
                        }
                    },
                    AuthState::LoggedIn(user) => rsx! {
                        div {
                            h2 { class: "text-xl font-semibold mb-4", "Willkommen!" }
                            UserProfile { user: user.clone() }
                        }
                    },
                    AuthState::Error(error) => rsx! {
                        div { class: "text-center",
                            div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                                strong { "Fehler: " }
                                span { "{error}" }
                            }
                            LoginButton {}
                        }
                    },
                }
            }
        }
    }
}

fn main() {
    launch(App);
}
