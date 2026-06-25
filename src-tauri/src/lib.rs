mod m3u;
mod player;
mod speedrun;
pub mod speedtest;

use serde::Serialize;
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};

pub struct AppState {
    pub channels: Mutex<Vec<m3u::Channel>>,
}

// ── Commands ──────────────────────────────────────────────────────

#[tauri::command]
async fn load_channels(state: State<'_, AppState>) -> Result<Vec<m3u::Channel>, String> {
    let path = m3u::m3u_path().map_err(|e| e.to_string())?;
    let channels = m3u::parse_m3u(&path).map_err(|e| e.to_string())?;
    *state.channels.lock().unwrap() = channels.clone();
    Ok(channels)
}

#[tauri::command]
async fn has_local_source() -> bool {
    m3u::m3u_path()
        .map(|p| p.exists() && p.metadata().map(|m| m.len() > 0).unwrap_or(false))
        .unwrap_or(false)
}

#[tauri::command]
async fn start_speedtest(workers: usize, top: usize, window: tauri::Window) -> Result<(), String> {
    speedrun::run(workers, top, window)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn play_url(url: String, title: String) -> Result<(), String> {
    player::play(&url, &title).map_err(|e| e.to_string())
}

#[tauri::command]
async fn stop_player() -> Result<(), String> {
    player::stop().map_err(|e| e.to_string())
}

// ── App entry ─────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            channels: Mutex::new(vec![]),
        })
        .invoke_handler(tauri::generate_handler![
            load_channels,
            has_local_source,
            start_speedtest,
            play_url,
            stop_player,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
