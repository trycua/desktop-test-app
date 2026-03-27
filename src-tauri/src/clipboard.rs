use arboard::Clipboard;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::state::{AppState, Event};

pub fn start_monitor(state: Arc<Mutex<AppState>>) {
    thread::spawn(move || {
        let Ok(mut cb) = Clipboard::new() else {
            eprintln!("[clipboard] failed to open clipboard — monitor disabled");
            return;
        };

        let mut last = String::new();

        loop {
            thread::sleep(Duration::from_millis(500));

            let current = cb.get_text().unwrap_or_default();
            if current != last {
                last = current.clone();
                let mut s = state.lock().unwrap();
                s.clipboard = current.clone();
                s.push(Event::new(
                    "clipboard_change",
                    serde_json::json!({ "text": current }),
                ));
            }
        }
    });
}
