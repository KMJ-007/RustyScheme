use std::{cell::RefCell, rc::Rc};

use linefeed::{Interface, ReadResult};

use crate::parser::Object;

mod env;
mod eval;
mod lexer;
mod parser;

const PROMPT: &str = "rusty-scheme> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    let mut env = Rc::new(RefCell::new(env::Env::new()));

    reader.set_prompt(PROMPT).as_ref().unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }
        let val = eval::eval(input.as_ref(), &mut env)?;
        match val {
            Object::Void => {}
            Object::Integer(n) => println!("{}", n),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{}", s),
            Object::Lambda(params, body) => {
                println!("Lambda(");
                for param in params {
                    println!("{:?}", param);
                }
                println!(")");
                for expr in body {
                    println!(" {:?}", expr);
                }
            }
            _ => print!("{:?}", val),
        }
    }
    print!("See you soon!");
    Ok(())
}
