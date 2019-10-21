use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};

use data_encoding::DecodeError;

#[derive(Debug)]
pub enum LorError {
    Decode(data_encoding::DecodeError),
    VarintDecode(std::io::Error),
    InvalidCardCode(std::num::ParseIntError),
    InvalidCard,
    InvalidDeck,
    Version,
}

impl fmt::Display for LorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            LorError::Decode(ref err) => <DecodeError as fmt::Display>::fmt(err, f),
            LorError::VarintDecode(ref err) => <std::io::Error as fmt::Display>::fmt(err, f),
            LorError::InvalidCardCode(ref err) => {
                <std::num::ParseIntError as fmt::Display>::fmt(err, f)
            }
            LorError::InvalidCard => write!(f, "Invalid card."),
            LorError::InvalidDeck => write!(f, "Invalid deck."),
            LorError::Version => write!(
                f,
                "The provided code requires a higher version of this library; please update."
            ),
        }
    }
}

impl Error for LorError {
    fn description(&self) -> &str {
        match *self {
            LorError::Decode(ref err) => err.description(),
            LorError::VarintDecode(ref err) => err.description(),
            LorError::InvalidCardCode(ref err) => err.description(),
            LorError::InvalidCard => "invalid card error",
            LorError::InvalidDeck => "invalid deck error",
            LorError::Version => "version error",
        }
    }
}

impl From<data_encoding::DecodeError> for LorError {
    fn from(err: data_encoding::DecodeError) -> Self {
        LorError::Decode(err)
    }
}

impl From<std::io::Error> for LorError {
    fn from(err: std::io::Error) -> Self {
        LorError::VarintDecode(err)
    }
}

impl From<std::num::ParseIntError> for LorError {
    fn from(err: std::num::ParseIntError) -> Self {
        LorError::InvalidCardCode(err)
    }
}
