# Contributing to Phatcake

Phatcake is currently pre-alpha. The open-source license is **not yet selected**, so external code contributions should wait until the license is approved and published.

Maintainer workflow:

1. Work on a dedicated branch and submit a pull request; protect `main` from unreviewed feature commits.
2. Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --all-targets`.
3. Do not allocate, lock, perform file or network I/O or wait for graphics rendering inside future audio callbacks.
4. Add deterministic tests for any transport, audio routing, resampling or mixing change. Performance claims require real hardware measurements.
5. Do not commit secrets, third-party commercial audio samples, unapproved proprietary assets or client/personal data.
6. Treat experimental mockups and offline DSP separately from production software.
