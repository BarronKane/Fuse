use futures_util::stream::StreamExt;
use reqwest::Client;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::thread;

use crate::app::ChannelsForFfmpegThread;

#[derive(Debug, Clone)]
pub struct DownloadInfo {
    pub content_length: u64,
    pub downloaded: u64,
}

impl Default for DownloadInfo {
    fn default() -> Self {
        Self {
            content_length: 0,
            downloaded: 0,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FfmpegInfo {
    #[serde(skip)]
    client: Client,
    #[serde(skip)]
    pub channels_for_ffmpeg: Option<crate::app::ChannelsForFfmpegThread>,
    #[serde(skip)]
    pub download_info_tx: Option<flume::Sender<DownloadInfo>>,

    download_url: String,
    download_location: PathBuf,
    unpack_location: PathBuf,
    exe_location: PathBuf,

    content_length: u64,
    downloaded: u64,
}

impl Default for FfmpegInfo {
    fn default() -> Self {
        let mut _download_location = fuse_util::get_cwd().unwrap().join("ffmpeg.zip");
        print!("Download Location: {}", _download_location.clone().to_str().unwrap());

        let mut _unpack_location = fuse_util::get_cwd().unwrap();
        _unpack_location.join("ffmpeg");

        let mut _exe_location = fuse_util::get_cwd().unwrap();
        _exe_location.join("bin/ffmpeg.exe");

        let _client = Client::new();

        Self {
            client: _client,
            channels_for_ffmpeg: None,
            download_info_tx: None,

            download_url: "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
                .to_string(),
            download_location: _download_location,
            unpack_location: _unpack_location,
            exe_location: _exe_location,

            content_length: 0,
            downloaded: 0,
        }
    }
}

impl FfmpegInfo {
    pub fn download_ffmpeg(&mut self) -> &Self {
        let _client = Client::new();
        print!("Test File Location: {}", self.download_location.clone().to_str().unwrap());
        if self.download_info_tx.is_some() {
            let _download_info_tx = self.download_info_tx.take().unwrap();
            let _download_location = self.download_location.clone();
            let _download_url = self.download_url.clone();
            let handle = thread::spawn(async move || {
                get_ffmpeg(
                    _client,
                    _download_location,
                    _download_url,
                    _download_info_tx,
                )
                .await;
            });
        }

        return self;
    }

    pub fn run_ffmpeg_job(ffmpeg_info: &FfmpegInfo, args: Vec<String>) {
        //let command = Command::new(ffmpeg_info.exe_location).args(args).output().unwrap();
    }
}

pub async fn get_ffmpeg(
    client: Client,
    download_location: PathBuf,
    download_url: String,
    download_info_tx: flume::Sender<DownloadInfo>,
) -> Result<(), String> {
    if download_location.exists() {
        return Ok(());
    }

    let response = client
        .get(download_url.clone())
        .send()
        .await
        .or(Err(format!("Could not download ffmpeg!")))?;

    let total_size = response.content_length().ok_or(format!(
        "Failed to get content length from '{}'",
        &download_url.clone()
    ))?;

    let mut file = std::fs::File::create(&download_location.clone()).or(Err(format!(
        "Could not create archive file: '{}', do you have the right permissions?",
        &download_location.clone().to_str().unwrap()
    )))?;

    let mut stream = response.bytes_stream();
    let _content_length = total_size;

    while let Some(item) = stream.next().await {
        let mut download_info = DownloadInfo {
            content_length: _content_length,
            downloaded: 0,
        };
        let chunk = item.or(Err(format!("Error while downloading ffmpeg!")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file!")))?;

        download_info.downloaded = download_info.downloaded + (chunk.len() as u64);
        download_info_tx.send(download_info).unwrap();
    }

    return Ok(());
}

pub async fn get_ffmpet() -> Result<(), String> {
    return Ok(());
}
