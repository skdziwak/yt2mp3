pub mod yt2mp3;

use serde_json::Value;
use crate::yt2mp3::errors;
use crate::yt2mp3::youtube;
use crate::yt2mp3::youtube::{Video, Playlist};
use crate::yt2mp3::errors::Error;
use crate::yt2mp3::tools::apply_sed_expression;
use std::process::{Command, Stdio};
use std::io::Write;
use clap::{App, Arg};

fn main() {
    let matches = App::new("yt2mp3")
        .version("1.0")
        .author("Szymon Dziwak <skdziwak@gmail.com>")
        .about("Downloads mp3 files from YouTube using youtube-dl. Allows customizing mp3 metadata.")
        .arg(Arg::new("playlist")
            .short('c')
            .long("playlist")
            .value_name("playlist")
            .about("YouTube playlist link")
            .takes_value(true))
        .arg(Arg::new("video")
            .short('v')
            .long("video")
            .value_name("video")
            .about("YouTube video link")
            .takes_value(true))
        .arg(Arg::new("title")
            .short('t')
            .long("title")
            .value_name("title")
            .about("Sed expression for evaluating mp3 title. Input format: 'ID__CHANNEL__TITLE'")
            .takes_value(true)
            .default_value("s/^.+__.+__(.+)$/$1/"))
    .arg(Arg::new("artist")
            .long("artist")
            .value_name("artist")
            .about("Sed expression for evaluating mp3 artist. Input format: 'ID__CHANNEL__TITLE'")
            .takes_value(true)
            .default_value("s/^.+__(.+)__.+$/$1/"))
        .arg(Arg::new("album")
            .long("album")
            .value_name("album")
            .about("Sed expression for evaluating mp3 album. Input format: 'ID__CHANNEL__TITLE'")
            .takes_value(true)
            .default_value("s/^.+$/NO ALBUM/"))
        .get_matches();
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
