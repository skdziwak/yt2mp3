use std::string::FromUtf8Error;

pub struct Error(pub String);

impl Error {
    pub fn from<S: Into<String>>(s: S) -> Error {
        Error(s.into())
    }
    pub fn to_string(&self) -> &String {
        return &self.0;
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error(err.to_string())
    }
}
