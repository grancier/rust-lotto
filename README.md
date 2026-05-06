# Rust Github Template

A template for [cargo generate](https://github.com/cargo-generate/cargo-generate) that aims to be a starting point suitable for
the vast majority of rust projects that will be hosted on GitHub.

See the project [website](https://rust-github.github.io).

## Testing the template

The repository itself is not a buildable Cargo project — `template/Cargo.toml`
contains `{{ ... }}` placeholders that only become valid TOML once
`cargo-generate` substitutes them. To validate changes to the template, generate
a project from it and run the same checks CI runs.

### With `cargo-generate` (preferred)

```sh
cargo install cargo-generate

# Generate a binary project from the local checkout.
cargo generate --path . --name my-bin --bin \
    -d project-description="test" -d gh-username=octocat

# Generate a library project.
cargo generate --path . --name my-lib --lib \
    -d project-description="test" -d gh-username=octocat
```

Then inside each generated project run the full CI suite:

```sh
cargo build --all-features --workspace
cargo test  --all-features --workspace
cargo fmt   --all --check
cargo clippy --all-targets --all-features --workspace -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc \
    --no-deps --document-private-items --all-features --workspace
```

### Verifying the declared MSRV

Generated projects declare `rust-version = "1.85"`. Verify the template still
builds on that toolchain:

```sh
rustup toolchain install 1.85.0 --profile minimal
cargo +1.85.0 check --all-features --workspace
```

### Without `cargo-generate`

If `cargo-generate` is unavailable, instantiate the template manually by
copying `template/Cargo.toml` and `template/src/{main,lib}.rs` into a temp
directory and replacing `{{project-name}}`, `{{project-description}}`, and
`{{gh-username}}` with concrete values. Then run the commands above. (The
files under `template/.github/` use Liquid `{% raw %}` blocks and are only
exercised by GitHub Actions in a generated project.)
