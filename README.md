# rust-lotto

A small command-line lottery number generator for four games:

| Game           | Main numbers      | Special ball         |
| -------------- | ----------------- | -------------------- |
| NJ Cash 5      | 5 of 1–45         | —                    |
| NJ Pick-6      | 6 of 1–46         | —                    |
| Mega Millions  | 5 of 1–70         | Mega Ball, 1–24      |
| Powerball      | 5 of 1–69         | Powerball, 1–26      |

Numbers are drawn without replacement from each pool, so the main numbers in
a single play are always unique. The Mega Ball and Powerball are drawn from
their own pools (matching the official lottery machines), so the special ball
may legally equal one of the main numbers.

Randomness is sourced from the operating system CSPRNG (`OsRng`), routed
through `rand`'s rejection-sampling APIs to keep the draw uniform and free of
modulo bias. A `--seed` flag is available for deterministic test output.

## Install

Requires Rust 1.85 or newer.

```sh
cargo install --path .
```

Or run directly from the checkout without installing:

```sh
cargo run --release -- <game> [--tickets N] [--seed N]
```

## Usage

```sh
rust-lotto <GAME> [-n N] [--seed S]
```

`<GAME>` is one of:

| Subcommand     | Alias |
| -------------- | ----- |
| `cash5`        | `5`   |
| `pick6`        | `6`   |
| `megamillions` | `m`   |
| `powerball`    | `p`   |

Options:

- `-n`, `--tickets <N>` — generate `N` independent plays (default `1`). Each
  play has its own pool reset, so the same number can appear across tickets.
- `--seed <S>` — seed the RNG for reproducible output (intended for tests).
  When omitted, the OS CSPRNG is used.

### Examples

```text
$ rust-lotto cash5
03 17 24 31 41

$ rust-lotto pick6 -n 3
04 11 19 22 38 45
01 09 14 27 33 40
05 18 21 25 36 44

$ rust-lotto megamillions
07 12 33 51 64  18

$ rust-lotto p -n 2
02 19 28 41 60  07
11 17 22 39 58  03
```

Main numbers are zero-padded to two digits and sorted ascending; the special
ball, when present, is separated by two spaces.

## Library

The crate also exposes a small library:

```rust
use rand::rngs::OsRng;
use rand::TryRngCore;
use rust_lotto::{draw, Game};

let mut rng = OsRng.unwrap_err();
let ticket = draw(Game::Powerball, &mut rng);
println!("{ticket}");
```

See `cargo doc --open` for the full API.

## Development

```sh
cargo build
cargo test --all-features --workspace
cargo fmt --all --check
cargo clippy --all-targets --all-features --workspace -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items --all-features --workspace
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.
