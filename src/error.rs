use thiserror::Error;

#[derive(Debug, Error)]
pub enum LorError {
    #[error("decode")]
    Decode(#[from] data_encoding::DecodeError),
    #[error("varint decode")]
    VarintDecode(#[from] std::io::Error),
    #[error("invalid card code")]
    InvalidCardCode(#[from] std::num::ParseIntError),
    #[error("invalid card")]
    InvalidCard,
    #[error("invalid deck")]
    InvalidDeck,
    #[error("version error")]
    Version,
}
