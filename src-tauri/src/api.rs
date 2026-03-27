use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use crate::state::AppState;

type SharedState = Arc<Mutex<AppState>>;

pub async fn serve(state: SharedState) {
    let app = Router::new()
        .route("/health", get(health))
        .route("/events", get(get_events))
        .route("/reset", post(reset_events))
        .route("/clipboard", get(get_clipboard))
        .route("/window-title", get(get_window_title))
        .route("/screen-size", get(get_screen_size))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:6769")
        .await
        .expect("failed to bind port 6769");

    // Signal ready — tests poll this
    println!("APP_HTTP_PORT=6769");

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn get_events(State(state): State<SharedState>) -> Json<Value> {
    let state = state.lock().unwrap();
    Json(json!(state.all_events()))
}

async fn reset_events(State(state): State<SharedState>) -> Json<Value> {
    let mut state = state.lock().unwrap();
    let cleared = state.reset();
    Json(json!({ "cleared": cleared }))
}

async fn get_clipboard(State(state): State<SharedState>) -> Json<Value> {
    let state = state.lock().unwrap();
    Json(json!({ "text": state.clipboard }))
}

async fn get_window_title(State(state): State<SharedState>) -> Json<Value> {
    let state = state.lock().unwrap();
    Json(json!({ "title": state.window_title }))
}

async fn get_screen_size(State(state): State<SharedState>) -> Json<Value> {
    let state = state.lock().unwrap();
    Json(json!({ "width": state.screen_width, "height": state.screen_height }))
}
