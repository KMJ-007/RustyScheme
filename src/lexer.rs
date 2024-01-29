pub fn hello_karan() {
    println!("hello Karan from lexer")
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

#[derive(Debug)]
pub struct TokenError {
    ch: char,
}

// tokenizer which will tokenize the program string into different tokens, it is what it is
pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    // adding space around the paren
    let program2 = program.replace("(", " ( ").replace(")", " ) ");
    // splitting the words with the help of white spaces
    let words = program2.split_whitespace();
    let mut tokens: Vec<Token> = Vec::new();
    // looping through each word and creating
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                // if token is int then push into that else it is symbol
                let i = word.parse::<i64>();
                if i.is_ok() {
                    tokens.push(Token::Integer(i.unwrap()));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
                }
            }
        }
    }
    // println!("this are the tokens");
    // println!("{:?}", tokens);
    Ok(tokens)
}

mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_add() {
        let tokens = tokenize("(+ 1 2)").unwrap_or(vec![]);
        println!("{:?}", tokens);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_area_of_square() {
        let program = "
        (
            (define side 7)
            (* side side)
        )
        ";
        let tokens = tokenize(program).unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("side".to_string()),
                Token::Integer(7),
                Token::RParen,
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("side".to_string()),
                Token::Symbol("side".to_string()),
                Token::RParen,
                Token::RParen
            ]
        );
    }

    #[test]
    fn test_no_paren() {
        let tokens = tokenize("+ 1 2").unwrap_or(vec![]);
        // it should return an err
        assert_eq!(
            tokens,
            vec![
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
            ]
        );
    }

    #[test]
    fn test_wrong_whitespace() {
        let tokens = tokenize("+   1     2 ").unwrap_or(vec![]);
        // it should return an err
        assert_eq!(
            tokens,
            vec![
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
            ]
        );
    }
}
