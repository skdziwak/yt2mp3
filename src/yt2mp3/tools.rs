use std::io::Write;
use std::process::{Command, Stdio};

use crate::errors::Error;

pub fn apply_sed_expression<S: Into<String>, D: Into<String>>(sed: S, input: D) -> Result<String, Error> {
    let sed = sed.into();
    let input = input.into();
    let input_b = input.into_bytes();

    let mut cmd = Command::new("sed")
        .arg("-r")
        .arg(sed)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdin = cmd.stdin.as_mut().ok_or(Error::from("Cannot get stdin."))?;
    stdin.write_all(&*input_b)?;
    let output = cmd.wait_with_output()?;
    let error = String::from_utf8(output.stderr)?;
    if error.is_empty() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(Error::from(error))
    }
}

pub fn apply_id3tool<S: Into<String>>(file: S, title: S, artist: S, album: S) -> Result<(), Error> {
    let output = Command::new("id3tool")
        .arg(file.into())
        .arg("-t").arg(title.into())
        .arg("-r").arg(artist.into())
        .arg("-a").arg(album.into())
        .output()?;
    let stderr = String::from_utf8(output.stderr)?;
    return if stderr.is_empty() {
        Ok(())
    } else {
        Err(Error::from(stderr))
    };
}