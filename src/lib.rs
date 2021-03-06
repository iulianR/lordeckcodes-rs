//! Legends of Runeterra deck encoder and decoder.
//!
//! # Usage
//!
//! The encoding and decoding can be done by directly calling the static functions found on [`Encoder`].
//!
//! [`Encoder`]: encoder/struct.Encoder.html
//!
//! # Examples
//!
//! Obtain a deck from the provided code:
//!
//! ```
//! use lordeckcodes::encoder;
//!
//! let deck = encoder::deck_from_code(String::from(
//!     "CEBAEAIBAQTQMAIAAILSQLBNGUBACAIBFYDACAAHBEHR2IBLAEBACAIFAY",
//! ));
//! assert!(deck.is_ok());
//! ```
//!
//! Generate a code from the provided deck:
//! ```
//! use lordeckcodes::{encoder, CardCodeAndCount, Deck, LorError};
//!
//! fn main() -> Result<(), LorError> {
//!     let deck: Deck = [
//!         ("01SI015", 3),
//!         ("01SI044", 3),
//!         ("01SI048", 3),
//!         ("01SI054", 3),
//!         ("01FR003", 3),
//!         ("01FR012", 3),
//!         ("01FR020", 3),
//!         ("01FR024", 3),
//!         ("01FR033", 3),
//!         ("01FR036", 3),
//!         ("01FR039", 3),
//!         ("01FR052", 3),
//!         ("01SI005", 2),
//!         ("01FR004", 2),
//!     ]
//!     .iter()
//!     .collect();
//!
//!     let code = encoder::code_from_deck(&deck);
//!     assert!(code.is_ok());
//!     Ok(())
//! }
//! ```

#[macro_use]
extern crate lazy_static;

mod card;
mod deck;
mod error;

/// Provides encode and decode API calls.
pub mod encoder;

pub use self::card::CardCodeAndCount;
pub use self::deck::Deck;
pub use self::error::LorError;
