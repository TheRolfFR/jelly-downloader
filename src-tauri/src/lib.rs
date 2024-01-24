// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use tauri::{Manager, State};
use tauri_plugin_dialog;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::ShellExt;
use uuid::Uuid;

mod downloader_thread;
pub use downloader_thread::*;

#[tauri::command]
fn download(url: &str, app_handle: tauri::AppHandle, s_sender: State<SenderStorage<DownloadRequest>>) -> Result<String, ()> {
    let document_dir = app_handle.path().download_dir().map_err(|_| { () })?;
    let uuid = Uuid::new_v4().to_string();

    let sender = s_sender.0.lock().map_err(|_| { () })?;
    sender.send(DownloadRequest {
        url: url.to_string(),
        uuid: uuid.clone(),
        out: document_dir
    }).map_err(|_| { () })?;

    Ok(uuid)
}

#[tauri::command]
fn dialog( app_handle: tauri::AppHandle) {
    app_handle.dialog().file().pick_file(|folder| {
        dbg!(folder);
    })
}

#[tauri::command]
fn open(file_path: String, app_handle: tauri::AppHandle) {
    if let Err(err) = app_handle.shell().open(file_path, None) {
        dbg!(err);
    }
}

struct SenderStorage<T>(Mutex<Sender<T>>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (req_schan, req_rchan) = channel::<DownloadRequest>();
    let (evt_schan, evt_rchan) = channel::<DownloadEvent>();

    tauri::Builder::default()
        .manage(SenderStorage(Mutex::new(req_schan)) )
        .invoke_handler(tauri::generate_handler![download, dialog, open])
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                downloader_thread(req_rchan, evt_schan ).await
            });

            let app_handle = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Ok(output) = evt_rchan.recv() {
                        app_handle.emit("DownloadEvent", output).ok();
                    }
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
