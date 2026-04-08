mod api;
mod clipboard;
mod state;

use state::{AppState, Event};
use std::sync::{Arc, Mutex};
use tauri::WebviewUrl;

/// Called from JS for every input event captured in the webview.
#[tauri::command]
fn log_event(
    state: tauri::State<Arc<Mutex<AppState>>>,
    event_type: String,
    details: serde_json::Value,
) {
    let mut s = state.lock().unwrap();
    s.push(Event::new(event_type, details));
}

/// Called once on startup to record the screen / window dimensions.
#[tauri::command]
fn set_screen_size(
    state: tauri::State<Arc<Mutex<AppState>>>,
    width: u32,
    height: u32,
    title: String,
) {
    let mut s = state.lock().unwrap();
    s.screen_width = width;
    s.screen_height = height;
    s.window_title = title;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let shared = Arc::new(Mutex::new(AppState::default()));

    // Clipboard monitor thread
    clipboard::start_monitor(shared.clone());

    // HTTP API on a dedicated tokio runtime so it never blocks Tauri
    let api_state = shared.clone();
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(api::serve(api_state));
    });

    // CUA_LOAD_URL: navigate the webview to an arbitrary URL instead of the
    // bundled index.html.  When unset, behaves as before (event-logger UI).
    // Useful for proxy/MITM tests: set CUA_LOAD_URL=https://example.com.
    let url_env = std::env::var("CUA_LOAD_URL").ok();
    let webview_url = match &url_env {
        Some(u) => WebviewUrl::External(u.parse().expect("CUA_LOAD_URL is not a valid URL")),
        None => WebviewUrl::App("index.html".into()),
    };

    tauri::Builder::default()
        .manage(shared)
        .invoke_handler(tauri::generate_handler![log_event, set_screen_size])
        .setup(move |app| {
            tauri::WebviewWindowBuilder::new(app, "main", webview_url)
                .title("Desktop Test App")
                .width(1000)
                .height(700)
                .resizable(true)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
