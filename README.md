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

## Building from source on Ubuntu / Debian

These instructions assume a fresh Ubuntu 22.04 or 24.04 host (or any
recent Debian/apt-based system) with no Rust toolchain installed. Run
them as a regular user; only the `apt` step needs `sudo`.

### 1. Install system build prerequisites

```sh
sudo apt update
sudo apt install -y build-essential curl ca-certificates git pkg-config
```

What each package is for:

- **`build-essential`** — pulls in `gcc`, the C standard library headers, and
  `make`. Cargo links the final binary with the system C linker, so this is
  required even though rust-lotto itself is pure Rust.
- **`curl` + `ca-certificates`** — used by the rustup installer below.
- **`git`** — to clone the repository.
- **`pkg-config`** — not strictly needed for rust-lotto's current
  dependency set, but it is the standard companion to `build-essential` for
  Rust crates and avoids surprises if you add a dependency that wraps a
  system library later.

> Note: `apt install rustc cargo` ships an older Rust version on most
> Ubuntu releases and may be below this crate's MSRV (1.85). Prefer
> rustup, below.

### 2. Install the Rust toolchain via rustup

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
source "$HOME/.cargo/env"
```

The first command is the [official rustup
installer](https://rustup.rs/), invoked non-interactively (`-y`) and
pinned to the stable channel. It installs `rustup`, `cargo`, `rustc`,
and friends under `~/.cargo` and `~/.rustup`, and modifies your shell
profile so they are on `PATH` for new shells. The `source` line makes
them available in the *current* shell without having to log out and
back in.

Verify the install:

```sh
rustc --version   # rustc 1.85.0 or newer
cargo --version
```

If `rustc` reports older than 1.85.0, run `rustup update stable`.

### 3. Clone the repository

```sh
git clone https://github.com/grancier/rust-lotto.git
cd rust-lotto
```

### 4. Build

A debug build is the fastest to compile and is fine for trying things
out:

```sh
cargo build
./target/debug/rust-lotto powerball -n 3
```

A release build is small and fast (the `Cargo.toml` enables thin LTO,
single codegen unit, and symbol stripping for the release profile):

```sh
cargo build --release
./target/release/rust-lotto megamillions -n 3
```

The first build downloads and compiles roughly 30 transitive crates
(`clap`, `rand`, `rand_chacha`, and their dependencies) and takes about
30–60 seconds on a modern laptop. Subsequent builds reuse Cargo's
incremental cache and complete in under a second.

### 5. Install the binary system-wide for the current user

```sh
cargo install --path .
```

This compiles in release mode and copies the resulting binary to
`~/.cargo/bin/rust-lotto`, which rustup has already added to `PATH`. You
can then run it from anywhere:

```sh
rust-lotto cash5 -n 5
```

### 6. (Optional) Run the test suite

```sh
cargo test --all-features --workspace
```

To match the checks CI runs, also:

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features --workspace -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items --all-features --workspace
```

### Troubleshooting

- **`linker 'cc' not found`** — `build-essential` was not installed.
  Re-run step 1.
- **`error: package 'rust-lotto' cannot be built because it requires
  rustc 1.85 or newer`** — your toolchain is older than the project's
  MSRV. Run `rustup update stable`.
- **`curl: command not found`** during step 2 — install `curl` from
  step 1 first.
- **Cargo hangs on "Updating crates.io index"** behind a corporate
  proxy — set `HTTPS_PROXY` and `HTTP_PROXY` in the environment, or
  configure `[http] proxy` in `~/.cargo/config.toml`.

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
