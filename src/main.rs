mod payloads;

use crate::payloads::{ForkPayload, StarPayload};
use axum::body::Bytes;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use config::Config;
use serde::Deserialize;
use std::io;
use tracing::{debug, error, info, instrument};

#[derive(Deserialize, Debug)]
struct AppConfig {
    host: String,
    debug: bool,
    secret: String,
}

#[tokio::main]
async fn main() {
    let app_config = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("Settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("MOTIVATE_ME"))
        .build()
        .unwrap()
        .try_deserialize::<AppConfig>()
        .unwrap();

    tracing_subscriber::fmt().with_writer(io::stderr).init();

    let app = Router::new()
        .route("/", get(root))
        .route("/github_webhook", post(github_webhook));

    info!("{}: {}", "Starting web server in", app_config.host);
    let listener = tokio::net::TcpListener::bind(app_config.host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[instrument]
async fn github_webhook(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
    info!("new github webhook received");
    let event = headers.get("X-GitHub-Event").and_then(|v| v.to_str().ok());
    // add logs
    match event {
        Some("star") => match serde_json::from_slice::<StarPayload>(&body) {
            Ok(payload) => {
                debug!("star event processed");
                (StatusCode::OK, format!("Star event: {payload:?}"))
            }
            Err(e) => {
                error!("{}: {}", "star event can't be processed", e.to_string());
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid star payload: {e}"),
                )
            }
        },
        Some("fork") => match serde_json::from_slice::<ForkPayload>(&body) {
            Ok(payload) => {
                debug!("fork event processed");
                (StatusCode::OK, format!("Fork event: {payload:?}"))
            }
            Err(e) => {
                error!("{}: {}", "fork event can't be processed", e.to_string());
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid fork payload: {e}"),
                )
            }
        },
        Some(other) => (
            StatusCode::BAD_REQUEST,
            format!("Unsupported event type: {other}"),
        ),
        None => (
            StatusCode::BAD_REQUEST,
            "Missing X-GitHub-Event header".to_owned(),
        ),
    }
}
