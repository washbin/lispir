//! parser.rs
//!
//! Parser takes the vector of tokens and converts it into a recursive list structure
//! The recursive list structure is an in-memory representation of the Lisp program

use crate::{lexer::Token, object::Object};

pub fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, String> {
    match tokens.pop() {
        Some(Token::OpenParenthesis) => {
            let mut list = Vec::new();

            while let Some(token) = tokens.pop() {
                match token {
                    Token::Integer(n) => list.push(Object::Integer(n)),
                    Token::Symbol(s) => list.push(Object::Symbol(s)),
                    Token::OpenParenthesis => {
                        tokens.push(Token::OpenParenthesis);
                        let sub_list = parse_list(tokens)?;

                        list.push(sub_list);
                    }
                    Token::CloseParenthesis => {
                        return Ok(Object::List(list));
                    }
                }
            }

            Err("Insufficient tokens".to_string())
        }
        Some(token) => Err(format!("Expected (, found {:?}", token)),
        None => Err("Expected (, found EOF".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn parses_a_token_stream() {
        let mut tokenized = tokenize("(+ 1 2)").into_iter().rev().collect();
        let list = parse_list(&mut tokenized).unwrap();
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
    fn parses_nested_token_stream() {
        let mut tokenized = tokenize("(+ 1 (* 2 3))").into_iter().rev().collect();
        let list = parse_list(&mut tokenized).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("+".to_string()),
                Object::Integer(1),
                Object::List(vec![
                    Object::Symbol("*".to_string()),
                    Object::Integer(2),
                    Object::Integer(3)
                ])
            ])
        )
    }
}
