#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::encoder::MAX_KNOWN_VERSION;
use crate::error::LorError;
use std::collections::HashMap;

lazy_static! {
    static ref FACTION_TO_INT: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("DE", 0);
        map.insert("FR", 1);
        map.insert("IO", 2);
        map.insert("NX", 3);
        map.insert("PZ", 4);
        map.insert("SI", 5);
        map.insert("BW", 6);
        map.insert("SH", 7);
        map.insert("MT", 9);
        map.insert("BC", 10);
        map.insert("RU", 12);
        map
    };
}

lazy_static! {
    static ref FACTION_NUMBER_TO_VERSION: HashMap<u32, u8> = {
        let mut map = HashMap::new();
        map.insert(0, 1);
        map.insert(1, 1);
        map.insert(2, 1);
        map.insert(3, 1);
        map.insert(4, 1);
        map.insert(5, 1);
        map.insert(6, 2);
        map.insert(9, 2);
        map.insert(7, 3);
        map.insert(10, 4);
        map.insert(12, 5);
        map
    };
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Hash, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct Card {
    set: u32,
    faction: u32,
    number: u32,
}

impl Card {
    pub(crate) fn new(set: u32, faction: u32, number: u32) -> Card {
        Card {
            set,
            faction,
            number,
        }
    }

    pub(crate) fn from_code(code: &str) -> Result<Card, LorError> {
        let faction = FACTION_TO_INT.get(&code[2..4]);
        if faction.is_none() {
            return Err(LorError::InvalidCard);
        }

        Ok(Card {
            set: (&code[0..2]).parse()?,
            faction: *faction.unwrap(),
            number: (&code[4..7]).parse()?,
        })
    }

    pub(crate) fn get_version(&self) -> u8 {
        let version = FACTION_NUMBER_TO_VERSION.get(&self.faction);

        if let Some(v) = version {
            *v
        } else {
            MAX_KNOWN_VERSION
        }
    }

    pub fn set(&self) -> u32 {
        self.set
    }

    pub fn faction(&self) -> u32 {
        self.faction
    }

    pub fn number(&self) -> u32 {
        self.number
    }
}

/// Stores card-related information.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Hash, Clone, Ord, PartialOrd, Eq)]
pub struct CardCodeAndCount {
    card: Card,
    count: i32,
}

impl CardCodeAndCount {
    pub(crate) fn new(card: Card, count: i32) -> CardCodeAndCount {
        CardCodeAndCount { card, count }
    }

    /// Create a `CardCodeAndCount` from the provided code and card count.
    pub fn from_data(code: &str, count: i32) -> Result<CardCodeAndCount, LorError> {
        if code.len() != 7 {
            return Err(LorError::InvalidCard);
        }

        if count < 1 {
            return Err(LorError::InvalidCard);
        }

        Ok(CardCodeAndCount {
            card: Card::from_code(code)?,
            count,
        })
    }

    pub fn card(&self) -> &Card {
        &self.card
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}
