#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::card::{Card, CardCodeAndCount};
use crate::error::LorError;

/// Holds a set of [`CardCodeAndCount`].
///
/// [`CardCodeAndCount`]: struct.CardCodeAndCount.html
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct Deck(Vec<CardCodeAndCount>);

impl Deck {
    /// Create a new empty `Deck`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new `Deck` from a `Vec` of `CardCodeAndCount`.
    ///
    /// # Examples
    /// ```
    /// use lordeckcodes::{Deck, CardCodeAndCount};
    /// let deck = Deck::from_vec(vec![
    ///     CardCodeAndCount::from_data("01SI015", 3).unwrap(),
    ///     CardCodeAndCount::from_data("01SI044", 3).unwrap(),
    ///     CardCodeAndCount::from_data("01SI048", 3).unwrap(),
    ///     CardCodeAndCount::from_data("01SI054", 3).unwrap(),
    /// ]);
    /// ```
    pub fn from_vec(vec: Vec<CardCodeAndCount>) -> Deck {
        Deck { 0: vec }
    }

    /// Add a `CardCodeAndCount` to the `Deck`.
    pub fn add(&mut self, card: CardCodeAndCount) {
        self.0.push(card);
    }

    /// Create and add a new `CardCodeAndCount` to the deck from the provided data.
    pub fn add_from_data(&mut self, code: &str, count: i32) -> Result<(), LorError> {
        let card = CardCodeAndCount::new(Card::from_code(code)?, count);
        self.add(card);
        Ok(())
    }

    /// Obtain a reference to the list of `CardCodeAndCount`.
    pub fn cards(&self) -> &Vec<CardCodeAndCount> {
        &self.0
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
