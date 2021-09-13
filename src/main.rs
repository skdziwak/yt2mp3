pub mod yt2mp3;

use serde_json::Value;
use crate::yt2mp3::errors;
use crate::yt2mp3::youtube;
use crate::yt2mp3::youtube::{Video, Playlist};
use crate::yt2mp3::errors::Error;
use crate::yt2mp3::sed::apply_sed_expression;
use std::process::{Command, Stdio};
use std::io::Write;
use clap::{AppSettings, Clap};

fn main() {
    let result = apply_sed_expression("s/^.*___(.*)___.*$/\\1/", "TEST___123___456");
    match result {
        Ok(result) => {
            println!("{}", result);
        }
        Err(_) => {}
    }
    // let playlist = youtube::Playlist::from_url("https://www.youtube.com/watch?v=w2yiK8xG_H8&list=OLAK5uy_luDy14PAkxNmVduDD-vUojQpWikN7OM44");
    // match playlist {
    //     Ok(playlist) => {
    //         for video in playlist.videos {
    //             println!("{}", video.download_mp3().is_ok())
    //         }
    //     }
    //     Err(err) => {
    //         eprintln!("{}", err.to_string())
    //     }
    // }
}
