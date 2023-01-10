#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rustube::*;
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

#[tauri::command]
async fn download_video(url: &str, path: &str) -> Result<(), ()> {
    let id = Id::from_raw(url).expect("REASON");
    let video = Video::from_id(id.into_owned()).await.expect("REASON");
    println!("Start Download Video To {}", path);
    video
        .best_quality()
        .unwrap()
        .download_to(path)
        .await
        .expect("REASON");

    println!("Downloaded video to {}", path);

    return Result::Ok(());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_version,
            video_info,
            download_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
