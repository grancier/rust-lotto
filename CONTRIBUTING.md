# Contributing

Thanks for your interest in contributing to rust-lotto.

For non-trivial changes, please open an issue first to discuss the approach
before sending a pull request.

## Reporting issues

Before opening an issue, please search the
[issue tracker](https://github.com/grancier/rust-lotto/issues) for related
reports. When filing a new issue, use a descriptive title and include enough
context to reproduce the behavior.

## Pull requests

- Keep each pull request focused on a single change.
- Ensure `cargo test`, `cargo fmt --all --check`, and
  `cargo clippy --all-targets --all-features --workspace -- -D warnings` all
  pass before requesting review.
- Add tests for new behavior where it makes sense.
