Jane-Eyre
=========

[![Latest Version](https://img.shields.io/crates/v/eyre.svg)](https://crates.io/crates/eyre)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/eyre)

```toml
[dependencies]
eyre = "0.1"
```

<br>

## Example

```rust
fn eyre::{err, ErrReport};

fn find_git_root() -> Result<PathBuf, ErrReport> {
    find_dir_in_ancestors(".git")
        .ok_or(err!("Failed to find git dir in parent directories"))
        .note("This command can only be run in git directories")
}
```

<br>

## Details

- This library tries to serve much the same role as `anyhow`, and infact it
  started as a fork from anyhow and draws a great deal of inspiration from it.
  The main differences between this crate and anyhow are that the following.
  - This crate supports tracing-error SpanTrace for user defined async aware context
  - This crate does not mix the idea of context and errors, in anyhow using the
    `context` method adds an error to your error chain, in this crate using the
    Result combinators pushes a context object into a vec in the Context.
  - This crate does weird things with its macros for fun, they generate zero
    sized error types from strings at the cost of code gen size.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>



