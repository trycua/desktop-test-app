use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_EVENTS: usize = 2000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub timestamp_ms: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub details: serde_json::Value,
}

impl Event {
    pub fn new(event_type: impl Into<String>, details: serde_json::Value) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self {
            timestamp_ms,
            event_type: event_type.into(),
            details,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub events: VecDeque<Event>,
    pub clipboard: String,
    pub screen_width: u32,
    pub screen_height: u32,
    pub window_title: String,
}

impl AppState {
    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
        if self.events.len() > MAX_EVENTS {
            self.events.pop_front();
        }
    }

    pub fn reset(&mut self) -> usize {
        let n = self.events.len();
        self.events.clear();
        n
    }

    pub fn all_events(&self) -> Vec<Event> {
        self.events.iter().cloned().collect()
    }
}
