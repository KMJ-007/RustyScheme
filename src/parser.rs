use crate::lexer::*;
use std::fmt;

pub fn hello_karan() {
    println!("hello Karan from parser");
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

pub fn parse(program: &str) -> Result<Object, ParseError> {
    let token_result = tokenize(program);
    // should i handle this
    if token_result.is_err() {
        return Err(ParseError {
            err: format!("{:?}", token_result.err().unwrap()),
        });
    }
    let mut tokens = token_result.unwrap().into_iter().rev().collect();
    let parsed_list = parse_list(&mut tokens)?;
    Ok(parsed_list)
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
    // we took first token
    let token = tokens.pop();
    // let's check if it is lparen or not
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", token),
        });
    }
    // parsed token will be stored in the list
    let mut list: Vec<Object> = Vec::new();
    // loop all the tokens
    while let Some(t) = tokens.pop() {
        match t {
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                // right paren means program is done
                return Ok(Object::List(list));
            }
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
        }
    }

    Ok(Object::List(list))
}

mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let list = parse("(+ 1 2)").unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("+".to_string()),
                Object::Integer(1),
                Object::Integer(2)
            ])
        )
    }

    #[test]
    fn test_area_of_a_circle() {
        let program = "(
            (define r 10)
            (define pi 3)
            (* pi (* r r))
        )";
        let list = parse(program).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Integer(10)
                ]),
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::Integer(3)
                ]),
                Object::List(vec![
                    Object::Symbol("*".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::List(vec![
                        Object::Symbol("*".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Symbol("r".to_string()),
                    ]),
                ]),
            ])
        )
    }
}
