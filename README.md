# trustfall-rustc-adapter

This [Trustfall](https://github.com/obi1kenobi/trustfall/) adapter aims to make large parts of the [Rust compiler's output](https://rustc-dev-guide.rust-lang.org/overview.html) easily queryable by anyone with help from [the `rustc_*` crates](https://rustc-dev-guide.rust-lang.org/rustc-driver.html).

## TODO

### Interesting Queries to Implement

- "what is the MSRV of this piece of code?"
    - look for instances of `stable` and `const_stable`
    - consider looking for 
- "where is this function called?"
- recreate some Clippy lints
