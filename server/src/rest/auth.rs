use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::warn;

use crate::error::RestError;

pub const CREATE_PERMISSION: &str = "create:cook_and_run";
pub const READ_PERMISSION: &str = "read:cook_and_run";
pub const UPDATE_PERMISSION: &str = "update:cook_and_run";
pub const DELETE_PERMISSION: &str = "delete:cook_and_run";

#[derive(Debug)]
pub enum AuthUser {
    Anonymous,
    None,
    Id(String),
    AnyOf(Vec<String>),
    AllOf(Vec<String>),
}

pub trait AuthenticatedUser {
    fn user_id(&self) -> AuthUser;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub aud: Vec<String>,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwksKey {
    //pub kty: String,
    pub kid: String,
    //pub r#use: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwks {
    pub keys: Vec<JwksKey>,
}

#[derive(Clone, Debug)]
pub struct AuthState {
    pub auth0_domain: String,
    pub auth0_audience: String,
    pub jwks: Jwks,
}

impl AuthState {
    pub async fn new(domain: &str, audience: &str) -> Result<Self, String> {
        let jwks_url = format!("https://{}/.well-known/jwks.json", domain);
        let jwks: Jwks = reqwest::get(&jwks_url)
            .await
            .map_err(|e| format!("Error while requesting JWKS: {}", e.to_string()))?
            .json()
            .await
            .map_err(|e| format!("Error while parsing JWKS-Response: {}", e.to_string()))?;

        Ok(AuthState {
            auth0_domain: domain.to_string(),
            auth0_audience: audience.to_string(),
            jwks,
        })
    }

    fn get_key(&self, kid: &str) -> Option<&JwksKey> {
        self.jwks.keys.iter().find(|key| key.kid == kid)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, String> {
        let header = decode_header(token)
            .map_err(|e| format!("Error while decoding header: {}", e.to_string()))?;
        let kid = header
            .kid
            .ok_or_else(|| "Token has no key id".to_string())?;

        let key = self
            .get_key(&kid)
            .ok_or_else(|| format!("Kid id {} not found!", kid).to_string())?;

        let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)
            .map_err(|e| format!("Error while decoding JWKS-Data: {}", e.to_string()))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.auth0_audience]);
        validation.set_issuer(&[&format!("https://{}/", self.auth0_domain)]);

        let token_data = decode::<Claims>(token, &decoding_key, &validation).map_err(|e| {
            format!(
                "Error while validating Token and extracting claims: {}",
                e.to_string()
            )
        })?;

        Ok(token_data.claims)
    }

    pub fn has_permission(&self, claims: &Claims, required_permission: &str) -> bool {
        // Prüfe zuerst das permissions Array (Auth0 Standard)
        if claims
            .permissions
            .contains(&required_permission.to_string())
        {
            return true;
        }

        // Fallback: Prüfe scope string (OAuth2 Standard)
        if let Some(scope) = &claims.scope {
            return scope.split_whitespace().any(|s| s == required_permission);
        }

        false
    }
}

// Permission-basierte Middleware Factory
pub fn require_permission(
    permission: &'static str,
) -> impl Fn(
    State<crate::AppState>,
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>,
> + Clone {
    move |State(state): State<crate::AppState>, mut request: Request, next: Next| {
        Box::pin(async move {
            let auth_header = request
                .headers()
                .get("authorization")
                .and_then(|header| header.to_str().ok())
                .ok_or_else(|| {
                    warn!("Missing authorization header");
                    StatusCode::UNAUTHORIZED
                })?;
            let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
                warn!("Invalid authorization header format");
                StatusCode::UNAUTHORIZED
            })?;

            let claims = match state.auth.verify_token(token) {
                Ok(claims) => claims,
                Err(e) => {
                    warn!("Token validation error: {}", e);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            };

            if !state.auth.has_permission(&claims, permission) {
                warn!(
                    "Missing permission '{}' for user {}",
                    permission, claims.sub
                );
                return Err(StatusCode::FORBIDDEN);
            }

            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        })
    }
}

pub fn is_user_authenticated<T: AuthenticatedUser>(
    user: &T,
    c_user_id: Option<&str>,
) -> Result<(), RestError> {
    let auth_user = user.user_id();
    let is_autherised = match &auth_user {
        AuthUser::Anonymous => true,
        AuthUser::None => false,
        AuthUser::Id(id) => {
            if let Some(c_user_id) = c_user_id {
                c_user_id == *id
            } else {
                false
            }
        }
        AuthUser::AnyOf(ids) => ids.iter().any(|id| {
            if let Some(c_user_id) = c_user_id {
                c_user_id == *id
            } else {
                false
            }
        }),
        AuthUser::AllOf(items) => items.iter().all(|id| {
            if let Some(c_user_id) = c_user_id {
                c_user_id == *id
            } else {
                false
            }
        }),
    };

    if !is_autherised {
        warn!(
            "User \"{}\" is not authorized. The following auth user was expected: {:?}",
            if let Some(c_user_id) = c_user_id {
                c_user_id
            } else {
                "NOT SET"
            },
            auth_user
        );
        return Err(RestError::Unauthorized {
            message: "User is not authorized".to_string(),
        });
    }
    Ok(())
}
