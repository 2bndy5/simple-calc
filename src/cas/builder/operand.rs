//! A module for building and manipulating a single operand for an expression.

use super::super::Token;
use std::{fmt::Display, str::FromStr};

/// Represents a mathematical expression as a sequence of tokens.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Operand {
    /// The token being built.
    ///
    /// This is [`None`] if the operand is empty.
    /// If this is [`Some`], then the [`Token`] variant can
    /// only be [`Token::Number`] or [`Token::Ident`].
    token: Option<Token>,
}

impl Operand {
    /// Creates a new expression from a vector of tokens.
    pub fn new(token: Token) -> Self {
        Self { token: Some(token) }
    }

    /// Returns the current token of the operand.
    ///
    /// Returns an empty [`Token::Number`] if the operand [`Self::is_empty()`]`.
    pub fn get_token(&self) -> Token {
        self.token.clone().unwrap_or(Token::Number(String::new()))
    }

    /// Appends a string to the end of the current operand.
    ///
    /// This is a convenience method that wraps [`Self::push()`].
    pub fn push_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.push(ch);
        }
    }

    /// Appends a character to the end of the current operand.
    pub fn push(&mut self, ch: char) {
        match &mut self.token {
            Some(Token::Number(n)) => {
                if ch == '0' && n == "0" {
                    // Do nothing to prevent leading zeros.
                } else if ch == '-' {
                    if n.starts_with('-') {
                        *n = n.trim_start_matches('-').to_string();
                    } else {
                        *n = format!("-{}", n);
                    }
                } else if ch.is_ascii_digit() || ch == '.' {
                    n.push(ch);
                }
                if ch == '-' {
                    if n.starts_with('-') {
                        self.token = Some(Token::Number(n.trim_start_matches('-').to_string()));
                    } else {
                        self.token = Some(Token::Number(format!("-{}", n)));
                    }
                } else if (n != "0" && ch == '0') && (ch.is_ascii_digit() || ch == '.') {
                    n.push(ch);
                }
            }
            Some(Token::Ident(n)) if ch.is_alphanumeric() || ch == '_' => n.push(ch),
            None => {
                if ch.is_ascii_digit() {
                    self.token = Some(Token::Number(ch.to_string()));
                } else if ch.is_alphabetic() || ch == '_' {
                    self.token = Some(Token::Ident(ch.to_string()));
                }
                self.token = Some(Token::Number(ch.to_string()));
            }
            _ => {}
        }
    }

    /// Removes the last character from the operand.
    pub fn pop(&mut self) {
        match self.token {
            Some(Token::Number(ref mut n)) => {
                n.pop();
                if n.is_empty() {
                    self.token = None;
                }
            }
            Some(Token::Ident(ref mut n)) => {
                n.pop();
                if n.is_empty() {
                    self.token = None;
                }
            }
            _ => {}
        }
    }

    /// Returns `true` if the operand is empty.
    pub fn is_empty(&self) -> bool {
        self.token.is_none()
    }

    /// Returns `true` if the operand is a number that contains a decimal point.
    pub fn has_dot(&self) -> bool {
        self.token.as_ref().is_some_and(|t| match t {
            Token::Number(n) => n.contains('.'),
            _ => false,
        })
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token {
            Some(token) => write!(f, "{token}"),
            _ => Ok(()),
        }
    }
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut token = Token::Number(String::new());
        for c in s.chars() {
            match c {
                '0'..='9' => match &mut token {
                    Token::Number(n) | Token::Ident(n) => n.push(c),
                    _ => {}
                },
                '.' => match &mut token {
                    Token::Number(n) => n.push(c),
                    _ => return Err("Dot not allowed in identifier".to_string()),
                },
                '_' | 'a'..='z' | 'A'..='Z' => match &mut token {
                    Token::Ident(n) => n.push(c),
                    Token::Number(_) => {
                        return Err("Numbers are not alphabetic or an underscore".to_string());
                    }
                    _ => {}
                },
                '-' => match &mut token {
                    Token::Number(n) => {
                        if n.starts_with('-') {
                            *n = n.trim_start_matches('-').to_string();
                        } else {
                            *n = format!("-{}", n);
                        }
                    }
                    _ => return Err("Identifiers cannot be negated".to_string()),
                },
                _ => return Err(format!("Invalid character in operand: {c}")),
            }
        }
        Ok(Self { token: Some(token) })
    }
}
