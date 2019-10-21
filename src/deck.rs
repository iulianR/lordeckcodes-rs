use crate::card::{Card, CardCodeAndCount};
use crate::error::LorError;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<CardCodeAndCount>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cards: vec![] }
    }

    pub fn from_vec(vec: Vec<CardCodeAndCount>) -> Deck {
        Deck { cards: vec }
    }

    fn add(&mut self, card: CardCodeAndCount) {
        self.cards.push(card);
    }

    pub fn add_from_data(&mut self, code: &str, count: i32) -> Result<(), LorError> {
        let card = CardCodeAndCount::new(Card::from_code(code)?, count);
        Ok(self.add(card))
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
