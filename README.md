# Legends of Runeterra - Deck Encoder/Decoder
Legends of Runeterra deck encoder/decoder in Rust. Port of [LorDeckCodes](https://github.com/RiotGames/LoRDeckCodes.).

## Documentation

[API Documentation](https://docs.rs/lordeckcodes)

## Usage

To use `lordeckcodes`, add this to your `Cargo.toml`:

```toml
[dependencies]
lordeckcodes = "0.1"
```

## Examples
 Obtain a deck from the provided code:

 ```
 use lordeckcodes::Encoder;

 let deck = Encoder::deck_from_code(
     String::from("CEBAEAIBAQTQMAIAAILSQLBNGUBACAIBFYDACAAHBEHR2IBLAEBACAIFAY")
 );
 assert!(deck.is_ok());
 ```

 Generate a code from the provided deck:
 ```
 use lordeckcodes::Encoder;
 use lordeckcodes::CardCodeAndCount;
 use lordeckcodes::Deck;

 let deck = Deck::from_vec(vec![
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
 ]);

 let code = Encoder::code_from_deck(&deck);
 assert!(code.is_ok());
 ```