mod address;
mod cook_and_run;
mod course;
mod db;
pub mod error;
mod note;
mod plan;
mod point;
mod rest;
pub mod rest_error;
mod sharing;
mod team;

use axum::http::HeaderValue;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use tower_http::{
    cors::CorsLayer,
    trace::{self, TraceLayer},
};
use tracing::{debug, error, info, warn, Level};

use crate::{db::Database, rest::auth::AuthState};

const DEFAULT_DATABASE_URL: &str = "postgresql://postgres:mysecretpassword@localhost:5432/postgres";
const DEFAULT_ADDR: &str = "0.0.0.0:3000";
const DEFAULT_ALLOW_ORIGIN: &str = "http://localhost:8080";

#[derive(Clone)]
struct AppState {
    auth: AuthState,
    db: Database,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Loading environment variables...");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        warn!(operation = "Loading environment variable",
              variable = "DATABASE_URL",
            "Environment variable \"DATABASE_URL\" not found. Using default value \"{}\" instead. For security reasons, it is highly recommended to change this value.",
            DEFAULT_DATABASE_URL
        );
        DEFAULT_DATABASE_URL.to_string()
    });
    let auth0_domain = std::env::var("AUTH0_DOMAIN").unwrap_or_else(|_| {
        error!(
            operation = "Loading environment variable",
            variable = "AUTH0_DOMAIN",
            "Environment variable \"AUTH0_DOMAIN\" must be set!"
        );
        panic!()
    });
    let auth0_audience = std::env::var("AUTH0_AUDIENCE")
        .expect("Environment variable \"AUTH0_AUDIENCE\" must be set!");
    let addr = std::env::var("ADDR").unwrap_or_else(|_| {
        info!(operation = "Loading environment variable",
              variable = "ADDR",
            "Environment variable \"ADDR\" not found. Using default value \"{}\" instead. For security reasons, it is highly recommended to change this value.",
            DEFAULT_ADDR
        );
        DEFAULT_ADDR.to_string()
    });
    let allow_origin =
        std::env::var("ALLOW_ORIGIN").map(|origin|
    vec![DEFAULT_ALLOW_ORIGIN.parse().unwrap(),
    origin.parse().unwrap()
])
.unwrap_or_else(|_| {
                    warn!(operation = "Loading environment variable",
              variable = "ALLOW_ORIGIN",
            "Environment variable \"ALLOW_ORIGIN\" not found. Using default value \"{}\" instead. For security reasons, it is highly recommended to change this value.",
            DEFAULT_ALLOW_ORIGIN
        );
       vec![
    DEFAULT_ALLOW_ORIGIN.parse().unwrap()
]});

    info!("Starting server...");
    debug!("Initializing AuthState...");
    let auth = AuthState::new(&auth0_domain, &auth0_audience).await;
    if auth.is_err() {
        error!(
            operation = "Initialize AuthState",
            "Failed to initialize AuthState: {}",
            auth.unwrap_err()
        );
        panic!();
    }

    let auth = auth.unwrap();
    debug!("AuthState initialized successfully.");
    debug!("Initializing Database...");
    let database = Database::new(&database_url).await;
    if let Some(err) = database.as_ref().err() {
        error!(
            operation = "Initialize Database",
            "Failed to initialize Database: {}", err
        );
        panic!();
    }

    let database = database.unwrap();
    debug!("Database initialized successfully.");
    let app_state = AppState { auth, db: database };

    let cors_layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_origin(allow_origin)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = rest::get_routes(app_state.clone())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors_layer)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
