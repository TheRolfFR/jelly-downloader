// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use filetime::FileTime;
use tauri::{Manager, State};
use tauri_plugin_dialog;
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

#[cfg(mobile)]
use tauri_plugin_openfile::{OpenFileRequest, OpenfileExt};
#[cfg(mobile)]
use tauri_plugin_openfile::OpenUrlRequest;
#[cfg(desktop)]
use tauri_plugin_shell::ShellExt;

mod downloader_thread;
pub use downloader_thread::*;

fn final_dir(app_handle: tauri::AppHandle) -> Result<PathBuf, String> {
    app_handle.path().download_dir().map_err(|e| e.to_string())
}

#[tauri::command]
fn download(url: &str, app_handle: tauri::AppHandle, s_sender: State<SenderStorage<DownloadRequest>>) -> Result<String, ()> {
    let document_dir = final_dir(app_handle).map_err(|_| { () })?;
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
fn read_dir(evt_sender: State<SenderStorage<DownloadUpdate>>, app_handle: tauri::AppHandle) -> Result<(), ()> {
    let sender = evt_sender.0.lock().map_err(|_| { () })?;
    let dir = final_dir(app_handle).map_err(|_| { () })?;

    // check dir exists
    let entries = fs::read_dir(dir).map_err(|_| { () })?;

    for res_entry in entries {
        if let Ok(entry) = res_entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let path = entry.path().to_string_lossy().to_string();
                    let size = metadata.len() as usize;
                    let mtime = FileTime::from_last_modification_time(&metadata);
                    let timestamp = mtime.unix_seconds();

                    let event = DownloadEvent {
                        uuid: path.clone(),
                        path: path.into(),
                        filetime: timestamp.into(),
                        downloaded: size,
                        total_size: size.into(),
                        event: DownloadEventType::Done
                    };

                    sender.send(DownloadUpdate::Event(event)).ok();
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
fn open(file_path: String, app_handle: tauri::AppHandle) {
    #[cfg(desktop)]
    {
        if let Err(err) = app_handle.shell().open(file_path.clone(), None) {
            dbg!(err);
        }
    }
    #[cfg(mobile)]
    {
        if let Err(err) = app_handle.openfile().open_file(OpenFileRequest {
            path: file_path
        }) {
            dbg!(err);
        }
    }
}

#[tauri::command]
fn open_url(url: String, app_handle: tauri::AppHandle) {
    #[cfg(desktop)]
    {
        if let Err(err) = app_handle.shell().open(url.clone(), None) {
            dbg!(err);
        }
    }
    #[cfg(mobile)]
    {
        if let Err(err) = app_handle.openfile().open_url(OpenUrlRequest {
            url
        }) {
            dbg!(err);
        }
    }
}

struct SenderStorage<T>(Mutex<Sender<T>>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (req_schan, req_rchan) = channel::<DownloadRequest>();
    let (evt_schan, evt_rchan) = channel::<DownloadUpdate>();

    tauri::Builder::default()
        .manage(SenderStorage(Mutex::new(req_schan)) )
        .manage(SenderStorage(Mutex::new(evt_schan.clone())))
        .invoke_handler(tauri::generate_handler![download, dialog, open, read_dir, open_url])
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
        .plugin(tauri_plugin_openfile::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
