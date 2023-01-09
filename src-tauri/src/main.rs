#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use rustube::Id;
use rustube::VideoDetails;
use rustube::VideoFetcher;
use std::sync::Arc;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_version() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

#[tauri::command]
async fn video_info(url: &str) -> Result<Arc<VideoDetails>, ()> {
    let id = match Id::from_raw(url) {
        Result::Ok(id) => id,
        Result::Err(err) => {
            println!("Error: {}", err);
            return Result::Err(());
        }
    };
    let descrambler = VideoFetcher::from_id(id.into_owned())
        .expect("REASON")
        .fetch()
        .await
        .expect("REASON");
    let video_info = descrambler.video_info();
    return Result::Ok(video_info.player_response.video_details.clone());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_version, video_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
