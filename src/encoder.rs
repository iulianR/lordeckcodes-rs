use std::io::Cursor;

use varint::{VarintRead, VarintWrite};

use crate::card::{Card, CardCodeAndCount};
use crate::deck::Deck;
use crate::error::LorError;

/// Creates a [`Deck`] from a provided code.
///
/// [`Deck`]: struct.Deck.html
///
/// # Examples
/// ```
/// use lordeckcodes::encoder;
///
/// let deck = encoder::deck_from_code(
///     String::from("CEBAEAIBAQTQMAIAAILSQLBNGUBACAIBFYDACAAHBEHR2IBLAEBACAIFAY")
/// );
/// assert!(deck.is_ok());
/// ```
pub fn deck_from_code<S>(code: S) -> Result<Deck, LorError>
where
    S: AsRef<str>,
{
    let mut bytes = data_encoding::BASE32_NOPAD.decode(code.as_ref().as_bytes())?;

    if bytes.is_empty() {
        return Err(LorError::InvalidCard);
    }

    let _format = bytes[0] >> 4;
    let version = bytes[0] & 0xF;
    bytes.remove(0);
    if version > 1 {
        return Err(LorError::Version);
    }

    let mut cards = vec![];
    let mut cursor = Cursor::new(bytes);
    for i in (1..=3).rev() {
        let num_groups = cursor.read_unsigned_varint_32()?;

        for _j in 0..num_groups {
            let num_this_group = cursor.read_unsigned_varint_32()?;
            let set = cursor.read_unsigned_varint_32()?;
            let faction = cursor.read_unsigned_varint_32()?;

            for _k in 0..num_this_group {
                let card_num = cursor.read_unsigned_varint_32()?;
                let card_count = CardCodeAndCount::new(Card::new(set, faction, card_num), i);
                cards.push(card_count);
            }
        }
    }

    while cursor.position() < cursor.get_ref().len() as u64 {
        let count = cursor.read_unsigned_varint_32()?;
        let set = cursor.read_unsigned_varint_32()?;
        let faction = cursor.read_unsigned_varint_32()?;
        let number = cursor.read_unsigned_varint_32()?;

        let card_count = CardCodeAndCount::new(Card::new(set, faction, number), count as i32);
        cards.push(card_count);
    }

    Ok(Deck::from_vec(cards))
}

/// Generate a code from the provided [`Deck`].
///
/// [`Deck`]: struct.Deck.html
///
/// # Examples
/// ```
/// use lordeckcodes::{encoder, CardCodeAndCount, Deck};
///
/// let deck = Deck::from_vec(vec![
///     CardCodeAndCount::from_data("01SI015", 3).unwrap(),
///     CardCodeAndCount::from_data("01SI044", 3).unwrap(),
///     CardCodeAndCount::from_data("01SI048", 3).unwrap(),
///     CardCodeAndCount::from_data("01SI054", 3).unwrap(),
///     CardCodeAndCount::from_data("01FR003", 3).unwrap(),
///     CardCodeAndCount::from_data("01FR012", 3).unwrap(),
///     CardCodeAndCount::from_data("01FR020", 3).unwrap(),
///     CardCodeAndCount::from_data("01FR024", 3).unwrap(),
///     CardCodeAndCount::from_data("01FR033", 3).unwrap(),
///     CardCodeAndCount::from_data("01FR036", 3).unwrap(),
///     CardCodeAndCount::from_data("01FR039", 3).unwrap(),
///     CardCodeAndCount::from_data("01FR052", 3).unwrap(),
///     CardCodeAndCount::from_data("01SI005", 2).unwrap(),
///     CardCodeAndCount::from_data("01FR004", 2).unwrap(),
/// ]);
///
/// let code = encoder::code_from_deck(&deck);
/// assert!(code.is_ok());
/// ```
pub fn code_from_deck(deck: &Deck) -> Result<String, LorError> {
    let mut bytes = vec![];
    bytes.push(0b0001_0001u8); // format and version

    let mut of3 = vec![];
    let mut of2 = vec![];
    let mut of1 = vec![];
    let mut ofn = vec![];

    for card_code in deck.cards() {
        match card_code.count() {
            3 => of3.push(card_code.clone()),
            2 => of2.push(card_code.clone()),
            1 => of1.push(card_code.clone()),
            n if n < 1 => return Err(LorError::InvalidDeck),
            _ => ofn.push(card_code.clone()),
        }
    }

    let mut grouped_of3 = group_by_set_and_faction(&mut of3);
    let mut grouped_of2 = group_by_set_and_faction(&mut of2);
    let mut grouped_of1 = group_by_set_and_faction(&mut of1);

    sort_group(&mut grouped_of3);
    sort_group(&mut grouped_of2);
    sort_group(&mut grouped_of1);

    ofn.sort_by_key(|x| x.count());

    bytes.extend(encode_group(&grouped_of3)?);
    bytes.extend(encode_group(&grouped_of2)?);
    bytes.extend(encode_group(&grouped_of1)?);
    bytes.extend(encode_rest(&ofn)?);

    Ok(data_encoding::BASE32_NOPAD.encode(&*bytes))
}

fn group_by_set_and_faction(cards: &mut Vec<CardCodeAndCount>) -> Vec<Vec<CardCodeAndCount>> {
    let mut result = vec![];

    while !cards.is_empty() {
        let ref_card_count = cards.remove(0);
        let ref_card = ref_card_count.card();

        let mut current_set = vec![];
        for card_count in cards.iter_mut() {
            if card_count.card().set() == ref_card.set()
                && card_count.card().faction() == ref_card.faction()
            {
                current_set.push(card_count.clone());
            }
        }

        cards.retain(|x| {
            !(x.card().set() == ref_card.set() && x.card().faction() == ref_card.faction())
        });

        current_set.push(ref_card_count);

        result.push(current_set);
    }

    result
}

fn sort_group(group: &mut Vec<Vec<CardCodeAndCount>>) {
    group.sort_by_key(|x| x.len());

    for cards in group {
        cards.sort_by_key(|x| x.card().number());
    }
}

fn encode_group(group: &[Vec<CardCodeAndCount>]) -> Result<Vec<u8>, LorError> {
    let bytes = vec![];
    let mut cursor = Cursor::new(bytes);
    cursor.write_unsigned_varint_32(group.len() as u32)?;

    for cards in group {
        cursor.write_unsigned_varint_32(cards.len() as u32)?;

        let ref_card = &cards.first().unwrap().card();
        cursor.write_unsigned_varint_32(ref_card.set())?;
        cursor.write_unsigned_varint_32(ref_card.faction())?;

        for card_count in cards {
            cursor.write_unsigned_varint_32(card_count.card().number())?;
        }
    }

    Ok(cursor.into_inner())
}

fn encode_rest(group: &[CardCodeAndCount]) -> Result<Vec<u8>, LorError> {
    let bytes = vec![];
    let mut cursor = Cursor::new(bytes);

    for card_count in group {
        cursor.write_unsigned_varint_32(card_count.count() as u32)?;
        cursor.write_unsigned_varint_32(card_count.card().set())?;
        cursor.write_unsigned_varint_32(card_count.card().faction())?;
        cursor.write_unsigned_varint_32(card_count.card().number())?;
    }

    Ok(cursor.into_inner())
}
