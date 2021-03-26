use std::io::Error as IoError;
use toml::de::Error as ParseError;
use bincode::Error as SerializeError;

#[derive(Debug)]
pub enum EntryError {
    NoEntry,
    IoError(IoError),
    ParseError(ParseError),
    SerializeError(SerializeError),
}

impl From<IoError> for EntryError {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}

impl From<ParseError> for EntryError {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<SerializeError> for EntryError {
    fn from(err: SerializeError) -> Self {
        Self::SerializeError(err)
    }
}

impl std::error::Error for EntryError {}

impl core::fmt::Display for EntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}