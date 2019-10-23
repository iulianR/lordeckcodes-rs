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
        map
    };
}

#[derive(PartialEq, Hash, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct Card {
    pub(crate) set: u32,
    pub(crate) faction: u32,
    pub(crate) number: u32,
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
}

/// Stores card-related information.
#[derive(Debug, PartialEq, Hash, Clone, Ord, PartialOrd, Eq)]
pub struct CardCodeAndCount {
    pub(crate) card: Card,
    pub(crate) count: i32,
}

impl CardCodeAndCount {
    pub(crate) fn new(card: Card, count: i32) -> CardCodeAndCount {
        CardCodeAndCount { card, count }
    }

    /// Create a `CardCodeAndCount` from the provided data.
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
}
