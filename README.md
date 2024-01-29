# rusty_scheme
A Lisp interpreter in Rust.

## Features
- **Lexing:** Converting the Lisp program text into a stream of tokens.
- **Parsing:** Converting the stream of tokens into an in-memory recursive list data structure.
- **Evaluation:** Walking the in-memory recursive list and producing the final result.

## Motivation
I completed half of the [Rust book](https://doc.rust-lang.org/book/) and 50% of [rustlings](https://github.com/rust-lang/rustlings), but I wasn't feeling confident. Although I started to get familiar with the syntax, I wanted to build something. So, I made this project.

This project would not have been possible without this [guide](https://vishpat.github.io/lisp-rs/overview.html).

why lisp ?
  - i read hackers and painters and want to understand it more
  - and this quote from [Rich Programmer Food](https://steve-yegge.blogspot.com/2007/06/rich-programmer-food.html)
  > "If you don't know how compilers work, then you don't know how computers work."

### Challenges and Learnings
This was my first time writing interpreter, during the development, I used ChatGPT and did a lot of Googling, redditing(is is real term?):

- I wanted to use the `println` macro for debugging tests, but for some reason, it wasn't working. The tests were passing, but `println` was not displaying anything.
  - **Solution:** By default, in tests, outputs are captured to keep the test output clean and focused only on what matters. To display the outputs during tests, you can run them like this:
    ```bash
    cargo test -- --nocapture
    ```
- your code may seem like exact to the solution or online forums, but if rust compiler is still saying no, then there is high chance that you forgot to implement trait, this will take some time to get used to it, you can add them in attributes array
  - want to debug or allow clone ? add `#[derive(Debug, Clone)]`
- asked about this [None and is_empty question](https://www.reddit.com/r/rust/comments/1acazl7/redundant_check_in_rust_parsing_loop_none_after/), people on reddit are really nice, they replied and taught me new things
- spend half hour debugging because i put semi colon after Ok at the end 😭
- learn about reference pointers this was lot of confusing, and i still think i only understand somewhat, still i haven't got intuitive understanding, but i managed to implement it
- got more used to rust syntax, and this was good then just watching and reading about rust on the internet

# features pending:
- supporting float values
- divide by zero and similar errors