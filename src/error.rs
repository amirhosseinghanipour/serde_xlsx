use std::fmt;

use serde::{de, ser};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Xml(quick_xml::Error),
    Zip(zip::result::ZipError),
    Serde(String),
    Calamine(calamine::Error),
    Custom(String),
    Xlsx(rust_xlsxwriter::XlsxError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Xml(e) => write!(f, "XML error: {}", e),
            Error::Zip(e) => write!(f, "ZIP error: {}", e),
            Error::Serde(e) => write!(f, "Serde error: {}", e),
            Error::Calamine(e) => write!(f, "Calamine error: {}", e),
            Error::Custom(e) => write!(f, "Custom error: {}", e),
            Error::Xlsx(e) => write!(f, "Xlsx error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        Error::Xml(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Error::Zip(err)
    }
}

impl From<calamine::Error> for Error {
    fn from(err: calamine::Error) -> Self {
        Error::Calamine(err)
    }
}

impl From<rust_xlsxwriter::XlsxError> for Error {
    fn from(err: rust_xlsxwriter::XlsxError) -> Self {
        Error::Xlsx(err)
    }
}
