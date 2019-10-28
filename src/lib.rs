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
//! let deck = encoder::deck_from_code(
//!     String::from("CEBAEAIBAQTQMAIAAILSQLBNGUBACAIBFYDACAAHBEHR2IBLAEBACAIFAY")
//! );
//! assert!(deck.is_ok());
//! ```
//!
//! Generate a code from the provided deck:
//! ```
//! use lordeckcodes::{CardCodeAndCount, Deck, encoder};
//!
//! let deck = Deck::from_vec(vec![
//!     CardCodeAndCount::from_data("01SI015", 3).unwrap(),
//!     CardCodeAndCount::from_data("01SI044", 3).unwrap(),
//!     CardCodeAndCount::from_data("01SI048", 3).unwrap(),
//!     CardCodeAndCount::from_data("01SI054", 3).unwrap(),
//!     CardCodeAndCount::from_data("01FR003", 3).unwrap(),
//!     CardCodeAndCount::from_data("01FR012", 3).unwrap(),
//!     CardCodeAndCount::from_data("01FR020", 3).unwrap(),
//!     CardCodeAndCount::from_data("01FR024", 3).unwrap(),
//!     CardCodeAndCount::from_data("01FR033", 3).unwrap(),
//!     CardCodeAndCount::from_data("01FR036", 3).unwrap(),
//!     CardCodeAndCount::from_data("01FR039", 3).unwrap(),
//!     CardCodeAndCount::from_data("01FR052", 3).unwrap(),
//!     CardCodeAndCount::from_data("01SI005", 2).unwrap(),
//!     CardCodeAndCount::from_data("01FR004", 2).unwrap(),
//! ]);
//!
//! let code = encoder::code_from_deck(&deck);
//! assert!(code.is_ok());
//! ```

#[macro_use]
extern crate lazy_static;

mod card;
mod deck;
mod error;

/// Provides encode and decode API calls.
pub mod encoder;

pub use self::deck::Deck;
pub use self::card::CardCodeAndCount;

