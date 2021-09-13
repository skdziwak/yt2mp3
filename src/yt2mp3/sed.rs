use crate::errors::Error;
use std::process::{Command, Stdio};
use std::io::Write;

pub fn apply_sed_expression<S: Into<String>>(sed: S, input: S) -> Result<String, Error> {
    let sed = sed.into();
    let input = input.into();
    let input_b = input.into_bytes();

    let mut cmd = Command::new("bash").arg("-c")
        .arg(format!("sed -r '{}'", sed))
        .stdin(Stdio::piped())
        .spawn()?;
    let mut stdin = cmd.stdin.as_mut().ok_or(Error::from("Cannot get stdin."))?;
    stdin.write_all(&*input_b);
    let output = cmd.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)?)
}