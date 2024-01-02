# trustfall-rustc-adapter

This [Trustfall](https://github.com/obi1kenobi/trustfall/) adapter aims to make large parts of the [Rust compiler's output](https://rustc-dev-guide.rust-lang.org/overview.html) easily queryable by anyone with help from [the `rustc_*` crates](https://rustc-dev-guide.rust-lang.org/rustc-driver.html).

## TODO

### Interesting Queries to Implement

- "what is the MSRV of this piece of code?"
    - look for instances of `stable` and `const_stable`
    - consider looking for 
- "where is this function called?"
- recreate some Clippy lints

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
