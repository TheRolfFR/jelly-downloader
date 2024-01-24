use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}, sync::mpsc::{Receiver, Sender}};
use serde::Serialize;
use tauri_plugin_http::reqwest;
use futures_util::StreamExt;

#[derive(Debug, Clone)]
pub struct DownloadRequest {
    pub uuid: String,
    pub url: String,
    pub out: PathBuf,
}

#[derive(Debug, Clone)]
pub enum DownloadEventType {
    Pending,
    Downloading(usize, usize),
    Done,
    Error
}

impl Serialize for DownloadEventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(format!("{:?}", self).as_str())
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct DownloadEvent {
    uuid: String,
    msg: Option<String>,
    path: Option<String>,
    event: DownloadEventType
}

pub async fn download_file(req: &DownloadRequest, evt_schan: &Sender<DownloadEvent>) -> Result<String, String> {
    dbg!(req);
    evt_schan.send(DownloadEvent {
        uuid: req.uuid.clone(),
        msg: None,
        event: DownloadEventType::Pending,
        path: None
    }).map_err(|e| e.to_string())?;

    let res = reqwest::get(&req.url).await.map_err(|e| e.to_string())?;

    let opt_attachment_filename = res.headers().get("content-disposition")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Some(s.split(";"))
        .and_then(|mut sp| sp.find(|s| s.starts_with("filename=")))
        .and_then(|s| s.trim().strip_prefix("filename="))
    );
    let url_filename = Path::new(&req.url).file_name().and_then(|f| f.to_str());
    let filename = opt_attachment_filename.or(url_filename).unwrap_or(&req.uuid);

    fs::create_dir_all(&req.out).map_err(|e| e.to_string())?;

    let out_file = req.out.as_path().join(filename);
    let out_file_string = (&out_file).to_string_lossy().into_owned();
    dbg!(&out_file);

    let total_size = res.content_length().unwrap_or_default() as usize;

    let mut file = File::create(out_file).map_err(|e| e.to_string())?;
    let mut downloaded = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;

        file.write_all(&chunk).map_err(|e| e.to_string())?;
        let new = std::cmp::min(downloaded + chunk.len(), total_size);
        downloaded = new;

        evt_schan.send(DownloadEvent {
            uuid: req.uuid.clone(),
            msg: None,
            event: DownloadEventType::Downloading(downloaded, total_size),
            path: Some(out_file_string.clone()),
        }).map_err(|e| e.to_string())?;
    }

    file.flush().map_err(|e| e.to_string())?;

    return Ok(out_file_string);
}

pub async fn downloader_thread(req_rchan: Receiver<DownloadRequest>, evt_schan: Sender<DownloadEvent>) -> ()
{
    loop {
        if let Ok(req) = req_rchan.recv() {
           let (event, msg, path) = match download_file(&req, &evt_schan).await {
                Ok(out_file) => (DownloadEventType::Done, None, Some(out_file)),
                Err(err) => (DownloadEventType::Error, Some(err), None),
            };

            evt_schan.send(DownloadEvent {
                uuid: req.uuid,
                event,
                msg,
                path,
            }).ok();
        }
    }
}
