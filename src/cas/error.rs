//! Error types for the CAS (Computer Algebra System) crate.

use super::Token;

/// Defines the error types for the CAS (Computer Algebra System) crate.
#[derive(Debug, thiserror::Error)]
pub enum CasError {
    /// An error that occurs when an preventing an undefined result.
    #[error("Undefined: {0}")]
    Undefined(String),

    /// An error that occurs when an unbound symbol is encountered during evaluation.
    #[error("Unbound symbol: {0}")]
    UnboundedSymbol(String),

    /// An error that occurs when an unknown function is called during parsing.
    #[error("Unknown function: '{0}'")]
    UnknownFunction(String),

    /// An error that occurs when an unexpected character is encountered during tokenization.
    #[error("Unexpected character: '{0}'")]
    #[cfg(test)]
    UnexpectedChar(char),

    /// An error that occurs when no tokens are provided for parsing.
    #[error("No tokens to parse")]
    NoTokens,

    /// An error that occurs when an token is encountered after a [`Token::Eof`].
    #[error("Trailing token: {0:?}")]
    TrailingToken(Token),

    /// An error that occurs when an unexpected token is encountered during parsing.
    #[error("Unexpected token: expected {expected:?}, found {found:?}")]
    UnexpectedToken {
        /// The expected token.
        expected: Token,
        /// The actual token that was found.
        found: Token,
    },

    /// An error that occurs when an unexpected token is encountered during parsing.
    #[error("Unexpected token in atomic expression: {0:?}")]
    UnexpectedTokenAtomic(Token),

    /// An error that occurs when a function is called with an incorrect number of arguments.
    #[error(
        "Inexact number of arguments for function '{function}': expected {expected}, found {found}"
    )]
    InexactNumberOfArguments {
        /// The name of the function being called.
        function: String,
        /// The expected number of arguments for the function.
        expected: usize,
        /// The actual number of arguments provided in the function call.
        found: usize,
    },

    /// An error that occurs when an invalid integer is encountered during evaluation.
    #[error("Invalid integer: '{literal}'")]
    InvalidInteger {
        /// The string representation of the invalid integer that caused the error.
        literal: String,
        /// The underlying error that occurred when parsing the integer.
        #[source]
        source: std::num::ParseIntError,
    },

    /// An error that occurs when an invalid float is encountered during evaluation.
    #[error("Invalid float: '{literal}'")]
    InvalidFloat {
        /// The string representation of the invalid float that caused the error.
        literal: String,
        /// The underlying error that occurred when parsing the float.
        #[source]
        source: std::num::ParseFloatError,
    },

    /// An error that occurs when an unsupported token is encountered during parsing.
    #[error("Unsupported token: '{0}'")]
    UnsupportedToken(String),
}
