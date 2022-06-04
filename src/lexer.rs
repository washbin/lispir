//! lexer.rs
//!
//! Lexer is a component that takes the program text and converts it to a stream of atomic units
//! known as tokens

/// Describes the possible tokens in our program
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    OpenParenthesis,
    CloseParenthesis,
    Integer(i64),
    Symbol(String),
}

/// Takes a program string as input and produces a vector of the tokens in the give
/// program.
///
/// # Example:
///
/// ```
/// use lispir::lexer::{tokenize, Token};
/// let token = tokenize("()");
/// assert_eq!(
///     token,
///     vec![
///         Token::OpenParenthesis,
///         Token::CloseParenthesis,
///     ]
/// );
/// ```
pub fn tokenize(program: &str) -> Vec<Token> {
    let program = program.replace('(', " ( ").replace(')', " ) ");
    let words = program.split_whitespace();

    let mut tokens = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::OpenParenthesis),
            ")" => tokens.push(Token::CloseParenthesis),
            _ => {
                if let Ok(value) = word.parse::<i64>() {
                    tokens.push(Token::Integer(value))
                } else {
                    tokens.push(Token::Symbol(word.to_string()))
                };
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_works_for_one_expression() {
        let tokens = tokenize("(+ 1 2)");
        assert_eq!(
            tokens,
            vec![
                Token::OpenParenthesis,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::CloseParenthesis
            ]
        );
    }
}
