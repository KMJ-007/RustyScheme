# RustyScheme
A Lisp interpreter in Rust.

## Features
- **Lexing:** Converting the Lisp program text into a stream of tokens.
- **Parsing:** Converting the stream of tokens into an in-memory recursive list data structure.
- **Evaluation:** Walking the in-memory recursive list and producing the final result.

## Motivation
I completed half of the [Rust book](https://doc.rust-lang.org/book/) and 50% of [rustlings](https://github.com/rust-lang/rustlings), but I wasn't feeling confident. Although I started to get familiar with the syntax, I wanted to build something. So, I made this project.

This project would not have been possible without this [guide](https://vishpat.github.io/lisp-rs/overview.html).

### Challenges and Learnings
During the development, I used ChatGPT and did a lot of Googling, especially for aspects of Rust I was unfamiliar with:

- I wanted to use the `println` macro for debugging tests, but for some reason, it wasn't working. The tests were passing, but `println` was not displaying anything.
  - **Solution:** By default, in tests, outputs are captured to keep the test output clean and focused only on what matters. To display the outputs during tests, you can run them like this:
    ```bash
    cargo test -- --nocapture
    ```
