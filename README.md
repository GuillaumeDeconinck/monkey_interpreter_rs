# Monkey language's intrepreter in Rust

This is my playground project for implementing an interpreter for the Monkey language. I'm following the [Writing an interpreter in Go](https://interpreterbook.com/) book, where the Monkey language comes from.

## Development

For launching the tests, you can run `cargo test` as usual.

Personally, I use [cargo-nextest](https://github.com/nextest-rs/nextest) as it's faster and provides a shorter output. The command with it is `cargo nextest run`.

Moreover, as the command is long to write, I use [just](https://github.com/casey/just) to easily launch it. With just, simply launch `just test`.
