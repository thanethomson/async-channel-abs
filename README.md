# Cross-Runtime `async` Channels

Recently I ran into a problem that required that we implement an abstraction to
cover synchronization channels (i.e. MPSC channels) from multiple runtimes (in
this case, Tokio and `async-std`).

Trouble is, it looks like you need [GATs] to be able to implement this, which
is currently only available on Rust Nightly.

[GATs]: https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md

## Requirements

* Rust Nightly (1.51+)

## Running the Tests

```bash
# Test for both Tokio and async-std
cargo test --all-features

# Test with just Tokio
cargo test --features with-tokio

# Test with just async-std
cargo test --features with-async-std
```
