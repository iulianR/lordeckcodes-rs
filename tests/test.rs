use std::fs::File;
use std::io::{BufRead, BufReader};

use lordeckcodes::CardCodeAndCount;
use lordeckcodes::Deck;
use lordeckcodes::Encoder;

#[test]
fn basic_decode_test() {
    let deck = Encoder::deck_from_code(String::from(
        "CEBAEAIBAQTQMAIAAILSQLBNGUBACAIBFYDACAAHBEHR2IBLAEBACAIFAY",
    ));
    assert!(deck.is_ok());
}

#[test]
fn basic_encode_test() {
    let cards = vec![
        CardCodeAndCount::from_data("01SI015", 3).unwrap(),
        CardCodeAndCount::from_data("01SI044", 3).unwrap(),
        CardCodeAndCount::from_data("01SI048", 3).unwrap(),
        CardCodeAndCount::from_data("01SI054", 3).unwrap(),
        CardCodeAndCount::from_data("01FR003", 3).unwrap(),
        CardCodeAndCount::from_data("01FR012", 3).unwrap(),
        CardCodeAndCount::from_data("01FR020", 3).unwrap(),
        CardCodeAndCount::from_data("01FR024", 3).unwrap(),
        CardCodeAndCount::from_data("01FR033", 3).unwrap(),
        CardCodeAndCount::from_data("01FR036", 3).unwrap(),
        CardCodeAndCount::from_data("01FR039", 3).unwrap(),
        CardCodeAndCount::from_data("01FR052", 3).unwrap(),
        CardCodeAndCount::from_data("01SI005", 2).unwrap(),
        CardCodeAndCount::from_data("01FR004", 2).unwrap(),
    ];
    let code = Encoder::code_from_deck(&Deck::from_vec(cards));
    assert_eq!(
        code.unwrap(),
        "CEBAIAIFB4WDANQIAEAQGDAUDAQSIJZUAIAQCBIFAEAQCBAA"
    );
}

#[test]
fn encode_decode_recommended() {
    let f = File::open("tests/DeckCodesTestData.txt").unwrap();
    let mut f = BufReader::new(f);

    let mut codes = vec![];
    let mut decks = vec![];

    loop {
        let mut line = String::new();
        let res = f.read_line(&mut line);

        if res.is_err() || res.unwrap() == 0 || line.is_empty() {
            break;
        }

        codes.push(line.trim_end_matches("\n").to_string());

        let mut deck = Deck::new();
        loop {
            let mut card_line = String::new();
            let res = f.read_line(&mut card_line);
            if res.is_err() || res.unwrap() == 0 || card_line == '\n'.to_string() {
                break;
            }

            let parts: Vec<&str> = card_line.trim_end_matches("\n").split(":").collect();
            deck.add_from_data(parts[1], parts[0].parse().unwrap())
                .unwrap();
        }
        decks.push(deck);
    }

    for (i, _deck) in decks.iter().enumerate() {
        assert_eq!(Encoder::code_from_deck(&decks[i]).unwrap(), codes[i]);
        assert!(verify_rehydration(
            &Encoder::deck_from_code(codes[i].to_string()).unwrap(),
            &decks[i]
        ));
    }
}

#[test]
fn small_deck() {
    let deck = Deck::from_vec(vec![CardCodeAndCount::from_data("01DE002", 1).unwrap()]);

    let code = Encoder::code_from_deck(&deck);
    let decoded_deck = Encoder::deck_from_code(code.unwrap());
    assert_eq!(deck, decoded_deck.unwrap());
}

#[test]
fn large_deck() {
    let deck = Deck::from_vec(vec![
        CardCodeAndCount::from_data("01DE002", 3).unwrap(),
        CardCodeAndCount::from_data("01DE003", 3).unwrap(),
        CardCodeAndCount::from_data("01DE004", 3).unwrap(),
        CardCodeAndCount::from_data("01DE005", 3).unwrap(),
        CardCodeAndCount::from_data("01DE006", 3).unwrap(),
        CardCodeAndCount::from_data("01DE007", 3).unwrap(),
        CardCodeAndCount::from_data("01DE008", 3).unwrap(),
        CardCodeAndCount::from_data("01DE009", 3).unwrap(),
        CardCodeAndCount::from_data("01DE010", 3).unwrap(),
        CardCodeAndCount::from_data("01DE011", 3).unwrap(),
        CardCodeAndCount::from_data("01DE012", 3).unwrap(),
        CardCodeAndCount::from_data("01DE013", 3).unwrap(),
        CardCodeAndCount::from_data("01DE014", 3).unwrap(),
        CardCodeAndCount::from_data("01DE015", 3).unwrap(),
        CardCodeAndCount::from_data("01DE016", 3).unwrap(),
        CardCodeAndCount::from_data("01DE017", 3).unwrap(),
        CardCodeAndCount::from_data("01DE018", 3).unwrap(),
        CardCodeAndCount::from_data("01DE019", 3).unwrap(),
        CardCodeAndCount::from_data("01DE020", 3).unwrap(),
        CardCodeAndCount::from_data("01DE021", 3).unwrap(),
    ]);

    let code = Encoder::code_from_deck(&deck);
    let decoded_deck = Encoder::deck_from_code(code.unwrap());
    assert!(verify_rehydration(&deck, &decoded_deck.unwrap()));
}

#[test]
fn deck_with_counts_more_than_3_small() {
    let deck = Deck::from_vec(vec![CardCodeAndCount::from_data("01DE002", 4).unwrap()]);

    let code = Encoder::code_from_deck(&deck);
    let decoded_deck = Encoder::deck_from_code(code.unwrap());
    assert!(verify_rehydration(&deck, &decoded_deck.unwrap()));
}

#[test]
fn deck_with_more_than_3_large() {
    let deck = Deck::from_vec(vec![
        CardCodeAndCount::from_data("01DE002", 3).unwrap(),
        CardCodeAndCount::from_data("01DE003", 3).unwrap(),
        CardCodeAndCount::from_data("01DE004", 3).unwrap(),
        CardCodeAndCount::from_data("01DE005", 3).unwrap(),
        CardCodeAndCount::from_data("01DE006", 4).unwrap(),
        CardCodeAndCount::from_data("01DE007", 5).unwrap(),
        CardCodeAndCount::from_data("01DE008", 6).unwrap(),
        CardCodeAndCount::from_data("01DE009", 7).unwrap(),
        CardCodeAndCount::from_data("01DE010", 8).unwrap(),
        CardCodeAndCount::from_data("01DE011", 9).unwrap(),
        CardCodeAndCount::from_data("01DE012", 3).unwrap(),
        CardCodeAndCount::from_data("01DE013", 3).unwrap(),
        CardCodeAndCount::from_data("01DE014", 3).unwrap(),
        CardCodeAndCount::from_data("01DE015", 3).unwrap(),
        CardCodeAndCount::from_data("01DE016", 3).unwrap(),
        CardCodeAndCount::from_data("01DE017", 3).unwrap(),
        CardCodeAndCount::from_data("01DE018", 3).unwrap(),
        CardCodeAndCount::from_data("01DE019", 3).unwrap(),
        CardCodeAndCount::from_data("01DE020", 3).unwrap(),
        CardCodeAndCount::from_data("01DE021", 3).unwrap(),
    ]);

    let code = Encoder::code_from_deck(&deck);
    let decoded_deck = Encoder::deck_from_code(code.unwrap());
    assert!(verify_rehydration(&deck, &decoded_deck.unwrap()));
}

#[test]
fn single_card_40_times() {
    let deck = Deck::from_vec(vec![CardCodeAndCount::from_data("01DE002", 40).unwrap()]);

    let code = Encoder::code_from_deck(&deck);
    let decoded_deck = Encoder::deck_from_code(code.unwrap());
    assert!(verify_rehydration(&deck, &decoded_deck.unwrap()));
}

#[test]
fn worst_case_length() {
    let deck = Deck::from_vec(vec![
        CardCodeAndCount::from_data("01DE002", 4).unwrap(),
        CardCodeAndCount::from_data("01DE003", 4).unwrap(),
        CardCodeAndCount::from_data("01DE004", 4).unwrap(),
        CardCodeAndCount::from_data("01DE005", 4).unwrap(),
        CardCodeAndCount::from_data("01DE006", 4).unwrap(),
        CardCodeAndCount::from_data("01DE007", 5).unwrap(),
        CardCodeAndCount::from_data("01DE008", 6).unwrap(),
        CardCodeAndCount::from_data("01DE009", 7).unwrap(),
        CardCodeAndCount::from_data("01DE010", 8).unwrap(),
        CardCodeAndCount::from_data("01DE011", 9).unwrap(),
        CardCodeAndCount::from_data("01DE012", 4).unwrap(),
        CardCodeAndCount::from_data("01DE013", 4).unwrap(),
        CardCodeAndCount::from_data("01DE014", 4).unwrap(),
        CardCodeAndCount::from_data("01DE015", 4).unwrap(),
        CardCodeAndCount::from_data("01DE016", 4).unwrap(),
        CardCodeAndCount::from_data("01DE017", 4).unwrap(),
        CardCodeAndCount::from_data("01DE018", 4).unwrap(),
        CardCodeAndCount::from_data("01DE019", 4).unwrap(),
        CardCodeAndCount::from_data("01DE020", 4).unwrap(),
        CardCodeAndCount::from_data("01DE021", 4).unwrap(),
    ]);

    let code = Encoder::code_from_deck(&deck);
    let decoded_deck = Encoder::deck_from_code(code.unwrap());
    assert!(verify_rehydration(&deck, &decoded_deck.unwrap()));
}

#[test]
fn order_should_not_matter_1() {
    let deck1 = Deck::from_vec(vec![
        CardCodeAndCount::from_data("01DE002", 1).unwrap(),
        CardCodeAndCount::from_data("01DE003", 2).unwrap(),
        CardCodeAndCount::from_data("02DE003", 3).unwrap(),
    ]);

    let deck2 = Deck::from_vec(vec![
        CardCodeAndCount::from_data("01DE003", 2).unwrap(),
        CardCodeAndCount::from_data("02DE003", 3).unwrap(),
        CardCodeAndCount::from_data("01DE002", 1).unwrap(),
    ]);

    assert_eq!(
        Encoder::code_from_deck(&deck1).unwrap(),
        Encoder::code_from_deck(&deck2).unwrap()
    );

    let deck3 = Deck::from_vec(vec![
        CardCodeAndCount::from_data("01DE002", 4).unwrap(),
        CardCodeAndCount::from_data("01DE003", 2).unwrap(),
        CardCodeAndCount::from_data("02DE003", 3).unwrap(),
    ]);

    let deck4 = Deck::from_vec(vec![
        CardCodeAndCount::from_data("01DE003", 2).unwrap(),
        CardCodeAndCount::from_data("02DE003", 3).unwrap(),
        CardCodeAndCount::from_data("01DE002", 4).unwrap(),
    ]);

    assert_eq!(
        Encoder::code_from_deck(&deck3).unwrap(),
        Encoder::code_from_deck(&deck4).unwrap()
    );
}

#[test]
fn order_should_not_matter_2() {
    let deck1 = Deck::from_vec(vec![
        CardCodeAndCount::from_data("01DE002", 4).unwrap(),
        CardCodeAndCount::from_data("01DE003", 2).unwrap(),
        CardCodeAndCount::from_data("02DE003", 3).unwrap(),
        CardCodeAndCount::from_data("01DE004", 5).unwrap(),
    ]);

    let deck2 = Deck::from_vec(vec![
        CardCodeAndCount::from_data("01DE004", 5).unwrap(),
        CardCodeAndCount::from_data("01DE003", 2).unwrap(),
        CardCodeAndCount::from_data("02DE003", 3).unwrap(),
        CardCodeAndCount::from_data("01DE002", 4).unwrap(),
    ]);

    assert_eq!(
        Encoder::code_from_deck(&deck1).unwrap(),
        Encoder::code_from_deck(&deck2).unwrap()
    );
}

#[test]
fn bad_card_codes() {
    assert!(CardCodeAndCount::from_data("01DE02", 1).is_err());
    assert!(CardCodeAndCount::from_data("01XX002", 1).is_err());
    assert!(CardCodeAndCount::from_data("01DE002", 0).is_err());
}

#[test]
fn bad_count() {
    assert!(CardCodeAndCount::from_data("01DE002", 0).is_err());
    assert!(CardCodeAndCount::from_data("01DE002", -1).is_err());
}

#[test]
fn garbage_decoding() {
    let bad_encoding_not_base32 = String::from("I'm no card code!");
    let bad_encoding32 = String::from("ABCDEFG");
    let bad_encoding_empty = String::from("");

    assert!(Encoder::deck_from_code(bad_encoding_not_base32).is_err());
    assert!(Encoder::deck_from_code(bad_encoding32).is_err());
    assert!(Encoder::deck_from_code(bad_encoding_empty).is_err());
}

fn verify_rehydration(d: &Deck, other: &Deck) -> bool {
    if d.cards().len() != other.cards().len() {
        return false;
    }

    for card_count in d.cards() {
        let res = other.cards().iter().find(|x| *x == card_count);
        if res.is_none() {
            return false;
        }
    }

    return true;
}
