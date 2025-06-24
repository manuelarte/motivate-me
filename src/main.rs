mod message_handler;
mod message_listener;
mod notifier;
mod payloads;
#[cfg(target_arch = "arm")]
mod raspberrypi_notifier;
mod signature_validator;

use crate::message_handler::{ActorMessage, MessageHandler};
use crate::message_listener::MessageListener;
use crate::payloads::{ForkPayload, StarPayload};
use crate::signature_validator::{SignatureValidator, get_signature_validator};
use axum::body::Bytes;
use axum::extract::State;
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
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, instrument};

#[derive(Debug)]
pub enum Error {
    GeneralError,
}

#[derive(Deserialize, Debug, Clone)]
struct AppConfig {
    debug: bool,
    environment: String,
    host: String,
    secret: String,
}

#[derive(Debug, Clone)]
struct AppState {
    signature_validator: Arc<dyn SignatureValidator>,
    actor_handler: Arc<MessageHandler>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_line_number(true)
        .init();

    let (tx, rx) = mpsc::channel::<ActorMessage>(1);
    let mut actor = MessageListener::new(rx);

    let actor_handler = MessageHandler::new(tx);

    let signature_validator = get_signature_validator(&app_config);
    let app_state = AppState {
        signature_validator,
        actor_handler: Arc::new(actor_handler),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/github_webhook", post(github_webhook))
        .with_state(app_state.clone());

    let backend = async move {
        info!("{}: {}", "Starting web server in", app_config.host);
        let listener = tokio::net::TcpListener::bind(app_config.host)
            .await
            .unwrap();
        axum::serve(listener, app).await
    };
    tokio::spawn(async move { actor.run().await });

    tokio::join!(backend);

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[instrument]
async fn github_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    info!("new github webhook received");
    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok());
    match signature {
        Some(signature) => {
            state
                .clone()
                .signature_validator
                .validate(body.iter().as_slice(), signature);
            let event = headers.get("X-GitHub-Event").and_then(|v| v.to_str().ok());
            // add logs
            match event {
                Some("star") => match serde_json::from_slice::<StarPayload>(&body) {
                    Ok(payload) => {
                        debug!("star event processed");
                        state.actor_handler.motivation_received().await;
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
                        state.actor_handler.motivation_received().await;
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
        None => {
            error!("no signature found");
            (StatusCode::BAD_REQUEST, "no signature found".to_string())
        }
    }
}
