# Legends of Runeterra - Deck Encoder/Decoder
Legends of Runeterra deck encoder/decoder in Rust. Port of [LorDeckCodes](https://github.com/RiotGames/LoRDeckCodes).

[![Actions Status](https://github.com/iulianR/lordeckcodes-rs/workflows/Rust/badge.svg)](https://github.com/iulianR/lordeckcodes-rs/actions)
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/lordeckcodes.svg
[crates-url]: https://crates.io/crates/lordeckcodes
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

## Documentation

[API Documentation](https://docs.rs/lordeckcodes)

## Usage

To use `lordeckcodes`, add this to your `Cargo.toml`:

```toml
[dependencies]
lordeckcodes = "0.3"
```

## Serde support

Serde support is optional and disabled by default. To enable use the feature `serde`.

```toml
[dependencies]
lordeckcodes = { version = "0.3", features = ["serde"] }
```

## Examples
Obtain a deck from the provided code:

```rust
use lordeckcodes::encoder;

let deck = encoder::deck_from_code("CEBAEAIBAQTQMAIAAILSQLBNGUBACAIBFYDACAAHBEHR2IBLAEBACAIFAY");
assert!(deck.is_ok());
```

Generate a code from the provided deck:
```rust
use lordeckcodes::{encoder, CardCodeAndCount, Deck, LorError};
fn main() -> Result<(), LorError> {
    let deck: Deck = [
        ("01SI015", 3),
        ("01SI044", 3),
        ("01SI048", 3),
        ("01SI054", 3),
        ("01FR003", 3),
        ("01FR012", 3),
        ("01FR020", 3),
        ("01FR024", 3),
        ("01FR033", 3),
        ("01FR036", 3),
        ("01FR039", 3),
        ("01FR052", 3),
        ("01SI005", 2),
        ("01FR004", 2),
    ]
    .iter()
    .collect();
    let code = encoder::code_from_deck(&deck);
    assert!(code.is_ok());
    Ok(())
}
```
