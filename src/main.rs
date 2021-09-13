use std::io::Write;
use std::process::{Command, Stdio};

use clap::{App, Arg, ArgMatches};
use serde_json::Value;

use crate::yt2mp3::errors;
use crate::yt2mp3::errors::Error;
use crate::yt2mp3::tools::{apply_id3tool, apply_sed_expression};
use crate::yt2mp3::youtube;
use crate::yt2mp3::youtube::{Playlist, Video};

pub mod yt2mp3;

fn process_video(video: &Video, title_sed: &str, album_sed: &str, artist_sed: &str) -> Result<(), Error> {
    let sed_input = video.to_sed_input_string();

    let title = apply_sed_expression(title_sed, &sed_input)?;
    let album = apply_sed_expression(album_sed, &sed_input)?;
    let artist = apply_sed_expression(artist_sed, &sed_input)?;

    let file_name = format!("{}.mp3", video.id);

    println!("Downloading mp3 file {}", file_name);
    video.download_mp3()?;

    println!("Setting metadata of {}", file_name);
    println!("\tTitle: {}", title);
    println!("\tAlbum: {}", album);
    println!("\tArtist: {}", artist);

    apply_id3tool(&file_name, &title, &artist, &album)?;

    println!("OK\n");
    Ok(())
}

fn run(matches: ArgMatches) -> Result<(), Error> {
    let title_sed = matches.value_of("title").unwrap();
    let album_sed = matches.value_of("album").unwrap();
    let artist_sed = matches.value_of("artist").unwrap();
    if matches.is_present("video") {
        let url = matches.value_of("video").unwrap();
        println!("Fetching video data");
        let video = Video::from_url(url)?;
        println!("Downloading video {}", video.id);
        process_video(&video, title_sed, album_sed, artist_sed)?;
    } else if matches.is_present("playlist") {
        let url = matches.value_of("playlist").unwrap();
        println!("Fetching playlist data");
        let playlist = Playlist::from_url(url)?;
        for video in &playlist.videos {
            println!("Downloading video {}", video.id);
            match process_video(video, title_sed, album_sed, artist_sed) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("{}", error.to_string());
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("yt2mp3")
        .version("1.0")
        .author("Szymon Dziwak <skdziwak@gmail.com>")
        .about("Downloads mp3 files from YouTube using youtube-dl. Allows customizing mp3 metadata.")
        .arg(Arg::new("playlist")
            .short('p')
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
            .default_value("s/^.+__.+__(.+)$/\\1/"))
        .arg(Arg::new("artist")
            .short('r')
            .long("artist")
            .value_name("artist")
            .about("Sed expression for evaluating mp3 artist. Input format: 'ID__CHANNEL__TITLE'")
            .takes_value(true)
            .default_value("s/^.+__(.+)__.+$/\\1/"))
        .arg(Arg::new("album")
            .short('a')
            .long("album")
            .value_name("album")
            .about("Sed expression for evaluating mp3 album. Input format: 'ID__CHANNEL__TITLE'")
            .takes_value(true)
            .default_value("s/^.+$/NO ALBUM/"))
        .get_matches();
    match run(matches) {
        Ok(_) => {
            println!("Done.");
        }
        Err(error) => {
            eprintln!("{}", error.to_string());
        }
    }
}
