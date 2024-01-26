use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}, sync::mpsc::{Receiver, Sender}};
use serde::Serialize;
use tauri_plugin_http::reqwest;
use futures_util::StreamExt;
use ts_rs::TS;

#[derive(Debug, Clone)]
pub struct DownloadRequest {
    pub uuid: String,
    pub url: String,
    pub out: PathBuf,
}

#[derive(TS)]
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DownloadEventType {
    #[default]
    Pending,
    Downloading,
    Done,
    Error
}

#[derive(TS, Clone, Serialize, Debug, Default)]
#[ts(export)]
pub struct DownloadEvent {
    pub uuid: String,
    #[ts(inline)]
    pub event: DownloadEventType,
    pub path: Option<String>,
    #[ts(type = "number")]
    pub downloaded: usize,
    #[ts(type = "number | null")]
    pub total_size: Option<usize>,
    #[ts(type = "number | null")]
    pub filetime: Option<i64>,
}

impl DownloadEvent {
    fn new(uuid: &str) -> Self {
        Self {
            uuid: uuid.to_string(),
            filetime: Some(filetime::FileTime::now().unix_seconds()),
            ..Default::default()
        }
    }
    pub fn done() -> Self {
        Self {
            event: DownloadEventType::Done,
            ..Default::default()
        }
    }

    fn to_downloading(mut self, total_size: Option<usize>, path: Option<String>) -> Self {
        self.event = DownloadEventType::Downloading;
        self.total_size = total_size;
        self.path = path;

        self
    }

    fn to_done(mut self, downloaded: usize) -> Self {
        self.event = DownloadEventType::Done;
        self.downloaded = downloaded;
        self.total_size = Some(downloaded);

        self
    }
}

#[derive(TS, Clone, Serialize, Debug)]
#[ts(export)]
pub struct DownloadError {
    uuid: String,
    msg: String,
}


#[derive(TS, Clone, Debug, Serialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
#[ts(export)]
pub enum DownloadUpdate {
    Event(DownloadEvent),
    #[ts(inline)]
    Error(DownloadError)
}

impl From<Result<DownloadEvent, DownloadError>> for DownloadUpdate {
    fn from(value: Result<DownloadEvent, DownloadError>) -> Self {
        match value {
            Ok(event) => DownloadUpdate::Event(event),
            Err(error) => DownloadUpdate::Error(error)
        }
    }
}

pub async fn download_file(req: &DownloadRequest, evt_schan: &Sender<DownloadUpdate>) -> Result<DownloadEvent, String> {
    let download_event = DownloadEvent::new(&req.uuid);
    evt_schan.send(DownloadUpdate::Event(download_event.clone())).map_err(|e| e.to_string())?;

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

    let opt_total_size = res.content_length().map(|s| s as usize);

    let mut file = File::create(out_file).map_err(|e| e.to_string())?;
    let mut downloaded = 0;
    let mut stream = res.bytes_stream();

    let mut progress_event = download_event.to_downloading(opt_total_size, Some(out_file_string));
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;

        file.write_all(&chunk).map_err(|e| e.to_string())?;

        downloaded = if let Some(total_size) = opt_total_size {
            std::cmp::min(downloaded + chunk.len(), total_size)
        } else {
            downloaded + chunk.len()
        };

        progress_event.downloaded = downloaded;

        evt_schan.send(DownloadUpdate::Event(progress_event.clone())).map_err(|e| e.to_string())?;
    }

    file.flush().map_err(|e| e.to_string())?;

    return Ok(progress_event.to_done(downloaded));
}

pub async fn downloader_thread(req_rchan: Receiver<DownloadRequest>, evt_schan: Sender<DownloadUpdate>) -> ()
{
    loop {
        if let Ok(req) = req_rchan.recv() {
           let result = match download_file(&req, &evt_schan).await {
                Ok(out_event) => Ok(out_event),
                Err(msg) => Err(DownloadError {
                    uuid: req.uuid.clone(),
                    msg,
                }),
            };

            evt_schan.send(result.into()).ok();
        }
    }
}
