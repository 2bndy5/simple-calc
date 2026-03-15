#![deny(missing_docs)]
//! A module for building/parsing/evaluating mathematical expressions.

pub mod builder;
pub mod display;
pub mod error;
pub mod eval;
pub mod expr;
pub mod lexer;
pub mod parser;

pub use builder::{ExprBuilder, Operand};
pub use eval::eval;
pub use expr::Expr;
pub use lexer::Token;
