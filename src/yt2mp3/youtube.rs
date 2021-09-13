use std::process::Command;

use serde_json::Value;

use crate::yt2mp3::errors::Error;

trait GetStringOrError {
    fn get_str_or_err(&self, key: &str) -> Result<String, Error>;
}

#[derive(Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub channel: String,
}

pub struct Playlist {
    pub videos: Vec<Video>,
}

pub fn get_yt_info(url: &str) -> Result<Value, Error> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("youtube-dl --skip-download --print-json '{}'", url))
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    if stderr.is_empty() {
        let value: Value = serde_json::from_str(stdout.as_str())?;
        return Ok(value);
    }
    Err(Error::from(stderr))
}

pub fn get_yt_multi_info(url: &str) -> Result<Vec<Value>, Error> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("youtube-dl --skip-download --print-json '{}'", url))
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    if stderr.is_empty() {
        let mut vec: Vec<Value> = Vec::new();
        for line in stdout.split("\n") {
            if !line.is_empty() {
                let value: Value = serde_json::from_str(line)?;
                vec.push(value);
            }
        }
        return Ok(vec);
    }
    Err(Error::from(stderr))
}

impl GetStringOrError for Value {
    fn get_str_or_err(&self, key: &str) -> Result<String, Error> {
        let str: String = self[key].to_string();
        if str == "null" || str.len() < 2 {
            return Err(Error(format!("{} is null", key)));
        }
        Ok(String::from(&str.as_str()[1..(str.len() - 1)]))
    }
}

impl Video {
    pub fn from_url<S: Into<String>>(url: S) -> Result<Video, Error> {
        let value = get_yt_info(url.into().as_str())?;
        Ok(Video {
            id: value.get_str_or_err("id")?,
            title: value.get_str_or_err("title")?,
            channel: value.get_str_or_err("channel")?,
        })
    }

    pub fn download_mp3(&self) -> Result<(), Error> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(format!("youtube-dl --extract-audio --audio-format mp3 -o '%(id)s.mp3' '{}'", self.id))
            .output()?;

        let stderr = String::from_utf8(output.stderr)?;
        return if stderr.is_empty() {
            Ok(())
        } else {
            Err(Error::from(stderr))
        }
    }
}

fn video_from_playlist_value(value: Value) -> Result<Video, Error> {
    Ok(Video {
        id: value.get_str_or_err("id")?,
        title: value.get_str_or_err("title")?,
        channel: value.get_str_or_err("uploader")?,
    })
}

impl Playlist {
    pub fn from_url<S: Into<String>>(url: S) -> Result<Playlist, Error> {
        let values = get_yt_multi_info(url.into().as_str())?;
        let mut videos: Vec<Video> = Vec::new();
        for value in values {
            let video = video_from_playlist_value(value);
            match video {
                Ok(video) => {videos.push(video);}
                Err(error) => {eprintln!("WARNING: Unable to read video data.\n{}", error.to_string())}
            }
        }
        Ok(Playlist { videos })
    }
}