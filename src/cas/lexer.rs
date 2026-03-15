//! Lexer for mathematical expressions.

use std::str::FromStr;

use super::error::CasError;

/// Tokens produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum Token {
    /// integer or float literal, e.g. "3", "2.5"
    ///
    /// Can start with a '-' for negative numbers, but
    /// [`tokenize()`] will recognize a `-` as an operator,
    /// not a prefix for a signed number.
    Number(String),
    /// symbol or function name, e.g. "x", "sin", "pi"
    ///
    /// Cannot start with a digit, but can contain digits after the first character.
    /// Can also contain underscores or lowercase/uppercase letters.
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Modulo,
    Factorial,
    LParen,
    RParen,
    Comma,
    Eof,
}

impl FromStr for Token {
    type Err = CasError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Token::Plus),
            "-" => Ok(Token::Minus),
            "*" => Ok(Token::Star),
            "/" => Ok(Token::Slash),
            "^" => Ok(Token::Caret),
            "%" => Ok(Token::Modulo),
            "!" => Ok(Token::Factorial),
            "(" => Ok(Token::LParen),
            ")" => Ok(Token::RParen),
            "," => Ok(Token::Comma),
            _ if s
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.' || (c == '-' && s.starts_with('-'))) =>
            {
                Ok(Token::Number(s.to_string()))
            }
            _ if s.chars().all(|c| c.is_alphanumeric() || c == '_')
                && !s.chars().next().is_some_and(|ch| ch.is_ascii_digit()) =>
            {
                Ok(Token::Ident(s.to_string()))
            }
            _ => Err(CasError::UnsupportedToken(s.to_string())),
        }
    }
}

/// Tokenizes `input` into a `Vec<Token>`.
///
/// Supports:
/// - integer and decimal literals (`3`, `3.14`)
/// - identifiers (`x`, `sin`, `pi`)
/// - operators `+  -  *  /  ^`
/// - parentheses and comma
#[cfg(test)]
pub fn tokenize(input: &str) -> Result<Vec<Token>, CasError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut num = String::new();
                let mut dot_seen = false;
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num.push(c);
                        chars.next();
                    } else if c == '.' && !dot_seen {
                        dot_seen = true;
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(ident));
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Caret);
                chars.next();
            }
            '%' => {
                tokens.push(Token::Modulo);
                chars.next();
            }
            '!' => {
                tokens.push(Token::Factorial);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            other => {
                return Err(CasError::UnexpectedChar(other));
            }
        }
    }
    tokens.push(Token::Eof);
    Ok(tokens)
}
