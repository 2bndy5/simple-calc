//! A module for building an expression from tokens.

use std::fmt::Display;

use super::super::{Expr, Token, error::CasError, parser::parse_from_tokens};

/// A builder for constructing an expression from a sequence of tokens.
#[derive(Debug, Clone, Default)]
pub struct ExprBuilder {
    tokens: Vec<Token>,
}

impl ExprBuilder {
    /// Builds an expression from the current tokens.
    pub fn build(&self) -> Result<Expr, CasError> {
        parse_from_tokens(self.tokens.clone())
    }

    /// Appends a token to the end of the builder.
    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    /// Removes the last token from the builder and returns it.
    pub fn pop(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    /// Returns a reference the last token in the builder.
    pub fn last(&self) -> Option<&Token> {
        self.tokens.last()
    }

    // /// Count number of unmatched brackets.
    // pub fn unmatched_brackets(&self) -> i32 {
    //     let mut count = 0;
    //     for token in &self.tokens {
    //         match token {
    //             Token::LParen => count += 1,
    //             Token::RParen => count -= 1,
    //             _ => {}
    //         }
    //     }
    //     count
    // }

    /// Clears all tokens from the builder.
    pub fn clear(&mut self) {
        self.tokens.clear();
    }
}

impl Display for ExprBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}
