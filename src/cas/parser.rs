//! Recursive-descent Pratt parser for infix expressions.
//!
//! Grammar (informal):
//! ```text
//! expr    := add_expr
//! add_expr:= mul_expr (('+' | '-') mul_expr)*
//! mul_expr:= unary   (('*' | '/') unary)*
//! unary   := '-' unary | power
//! power   := atom ('^' unary)?        ← right-associative
//! atom    := NUMBER | IDENT | IDENT '(' args ')' | '(' expr ')'
//! args    := expr (',' expr)*
//! ```

use super::{error::CasError, expr::Expr, lexer::Token};

/// Parse a list of tokens into an `Expr`.
///
/// If the given `tokens` are empty, then this returns an error.
///
/// If the last token is not [`Token::Eof`], then this appends an
/// [`Token::Eof`] token to the end of the list before parsing.
pub fn parse_from_tokens(tokens: Vec<Token>) -> Result<Expr, CasError> {
    if tokens.is_empty() {
        return Err(CasError::NoTokens);
    }
    let tokens = if let Some(t) = tokens.last()
        && t != &Token::Eof
    {
        // ensure EOF at end
        [tokens, vec![Token::Eof]].concat()
    } else {
        tokens
    };
    let mut p = Parser { tokens, pos: 0 };
    let expr = p.parse_expr()?;
    if p.current() != &Token::Eof {
        return Err(CasError::TrailingToken(p.current().clone()));
    }
    Ok(expr)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), CasError> {
        if self.current() == expected {
            self.advance();
            Ok(())
        } else {
            Err(CasError::UnexpectedToken {
                expected: expected.clone(),
                found: self.current().clone(),
            })
        }
    }

    // ------------------------------------------------------------------ rules

    fn parse_expr(&mut self) -> Result<Expr, CasError> {
        self.parse_add()
    }

    fn parse_add(&mut self) -> Result<Expr, CasError> {
        let mut lhs = self.parse_mul()?;

        loop {
            match self.current() {
                Token::Plus => {
                    self.advance();
                    let rhs = self.parse_mul()?;
                    lhs = match lhs {
                        Expr::Add(mut v) => {
                            v.push(rhs);
                            Expr::Add(v)
                        }
                        other => Expr::Add(vec![other, rhs]),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let rhs = self.parse_mul()?;
                    let neg_rhs = Expr::Neg(Box::new(rhs));
                    lhs = match lhs {
                        Expr::Add(mut v) => {
                            v.push(neg_rhs);
                            Expr::Add(v)
                        }
                        other => Expr::Add(vec![other, neg_rhs]),
                    };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_mul(&mut self) -> Result<Expr, CasError> {
        let mut lhs = self.parse_unary()?;

        loop {
            match self.current() {
                Token::Star => {
                    self.advance();
                    let rhs = self.parse_unary()?;
                    lhs = match lhs {
                        Expr::Mul(mut v) => {
                            v.push(rhs);
                            Expr::Mul(v)
                        }
                        other => Expr::Mul(vec![other, rhs]),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let rhs = self.parse_unary()?;
                    let inv = Expr::Pow(Box::new(rhs), Box::new(Expr::num(-1)));
                    lhs = match lhs {
                        Expr::Mul(mut v) => {
                            v.push(inv);
                            Expr::Mul(v)
                        }
                        other => Expr::Mul(vec![other, inv]),
                    };
                }
                Token::Modulo => {
                    self.advance();
                    let rhs = self.parse_unary()?;
                    lhs = Expr::Modulo(Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, CasError> {
        if self.current() == &Token::Minus {
            self.advance();
            let e = self.parse_unary()?;
            return Ok(Expr::Neg(Box::new(e)));
        }
        self.parse_power()
    }

    fn parse_power(&mut self) -> Result<Expr, CasError> {
        let base = self.parse_atom()?;
        if self.current() == &Token::Caret {
            self.advance();
            let exp = self.parse_unary()?; // right-assoc
            return Ok(Expr::Pow(Box::new(base), Box::new(exp)));
        }
        Ok(base)
    }

    fn parse_atom(&mut self) -> Result<Expr, CasError> {
        match self.current().clone() {
            Token::Number(s) => {
                self.advance();
                parse_number(&s)
            }
            Token::Ident(name) => {
                self.advance();
                // Check if this is a function call
                if self.current() == &Token::LParen {
                    self.advance(); // consume '('
                    let mut args = vec![self.parse_expr()?];
                    while self.current() == &Token::Comma {
                        self.advance();
                        args.push(self.parse_expr()?);
                    }
                    self.expect(&Token::RParen)?;
                    parse_function_call(&name, args)
                } else {
                    // Named constants
                    match name.as_str() {
                        "pi" => Ok(Expr::Sym("pi".into())),
                        "e" => Ok(Expr::Sym("e".into())),
                        _ => Ok(Expr::Sym(name)),
                    }
                }
            }
            Token::LParen => {
                self.advance();
                let e = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(e)
            }
            other => Err(CasError::UnexpectedTokenAtomic(other)),
        }
    }
}

// ------------------------------------------------------------------ helpers

fn parse_number(s: &str) -> Result<Expr, CasError> {
    if s.contains('.') {
        let float = s.parse::<f64>().map_err(|e| CasError::InvalidFloat {
            literal: s.to_string(),
            source: e,
        })?;
        Ok(Expr::Float(float))
    } else {
        let n: i64 = s.parse().map_err(|e| CasError::InvalidInteger {
            literal: s.to_string(),
            source: e,
        })?;
        Ok(Expr::num(n))
    }
}

fn parse_function_call(name: &str, mut args: Vec<Expr>) -> Result<Expr, CasError> {
    match name {
        "sqrt" => {
            if args.len() != 1 {
                return Err(CasError::InexactNumberOfArguments {
                    function: name.into(),
                    expected: 1,
                    found: args.len(),
                });
            }
            Ok(Expr::Pow(
                Box::new(args.remove(0)),
                Box::new(Expr::rat(1, 2)),
            ))
        }
        "exp" => {
            if args.len() != 1 {
                return Err(CasError::InexactNumberOfArguments {
                    function: name.into(),
                    expected: 1,
                    found: args.len(),
                });
            }
            Ok(Expr::Pow(
                Box::new(Expr::Sym("e".into())),
                Box::new(args.remove(0)),
            ))
        }
        other => Err(CasError::UnknownFunction(other.into())),
    }
}

#[cfg(test)]
pub(super) mod tests {
    #![allow(clippy::unwrap_used)]

    use super::super::{error::CasError, expr::Expr, lexer::tokenize};
    use super::parse_from_tokens;

    /// Parse a string into an `Expr`.
    pub fn parse(input: &str) -> Result<Expr, CasError> {
        let tokens = tokenize(input)?;
        parse_from_tokens(tokens)
    }

    #[test]
    fn parse_integer() {
        assert_eq!(parse("42").unwrap(), Expr::num(42));
    }

    #[test]
    fn parse_symbol() {
        assert_eq!(parse("x").unwrap(), Expr::Sym("x".into()));
    }

    #[test]
    fn parse_addition() {
        let e = parse("x + 1").unwrap();
        assert_eq!(e, Expr::Add(vec![Expr::Sym("x".to_string()), Expr::num(1)]));
    }

    #[test]
    fn parse_unary_minus() {
        let e = parse("-x").unwrap();
        assert_eq!(e, Expr::Neg(Box::new(Expr::Sym("x".to_string()))));
    }

    #[test]
    fn parse_power() {
        let e = parse("x^2").unwrap();
        assert_eq!(
            e,
            Expr::Pow(Box::new(Expr::Sym("x".to_string())), Box::new(Expr::num(2)))
        );
    }

    #[test]
    fn parse_function_sqrt() {
        let e = parse("sqrt(x)").unwrap();
        assert_eq!(
            e,
            Expr::Pow(
                Box::new(Expr::Sym("x".to_string())),
                Box::new(Expr::rat(1, 2))
            )
        );
    }

    #[test]
    fn parse_decimal() {
        let e = parse("0.5").unwrap();
        assert_eq!(e, Expr::rat(1, 2));
    }
}
