mod address;
mod cook_and_run;
mod course;
mod db;
pub mod error;
mod note;
mod plan;
mod rest;
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
use tracing::{debug, error, info, Level};

use crate::{db::Database, rest::auth::AuthState};

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
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    //  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/postgres".to_string());
    //let domain = std::env::var("DOMAIN").expect("DOMAIN must be set");
    //let audience = std::env::var("AUDIENCE").expect("AUDIENCE must be set");
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    info!("Starting server...");
    debug!("Initializing AuthState...");
    let auth = AuthState::new(
        "beancode.eu.auth0.com",
        "https://home.beancode.de/tcc/backend",
    )
    .await;
    if auth.is_err() {
        error!("Failed to initialize AuthState: {}", auth.unwrap_err());
        panic!("AuthState initialization failed");
    }

    let auth = auth.unwrap();
    debug!("AuthState initialized successfully.");
    debug!("Initializing Database...");
    let database = Database::new(&database_url).await;
    if let Some(err) = database.as_ref().err() {
        error!("Failed to initialize Database: {}", err);
        panic!("Database initialization failed");
    }

    let database = database.unwrap();
    debug!("Database initialized successfully.");
    let app_state = AppState { auth, db: database };

    let cors_layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
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
