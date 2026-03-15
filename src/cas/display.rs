//! Pretty-printing for `Expr`.
//!
//! Implements `std::fmt::Display` so you can do `println!("{}", expr)`.
//!
//! Precedence levels (higher = tighter):
//!   Add  → 1
//!   Neg  → 2
//!   Mul  → 3
//!   Pow  → 4
//!   Atom → 5

use super::{Token, expr::Expr};
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precursor {
    None,
    Add,
    Mul,
    Pow,
    Atom,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Ident(s) => write!(f, "{s}"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Caret => write!(f, "^"),
            Self::Modulo => write!(f, "%"),
            Self::Factorial => write!(f, "!"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::Comma => write!(f, ","),
            Self::Eof => write!(f, "="),
        }
    }
}

impl Precursor {
    fn expr_precursor(expr: &Expr) -> Self {
        match expr {
            Expr::Add(_) => Self::Add,
            Expr::Neg(_) => Self::Add,
            Expr::Mul(_) => Self::Mul,
            Expr::Pow(_, _) => Self::Pow,
            _ => Self::Atom,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            DisplayExpr {
                expr: self,
                precursor: Precursor::None
            }
        )
    }
}

struct DisplayExpr<'a> {
    expr: &'a Expr,
    precursor: Precursor,
}

impl<'a> fmt::Display for DisplayExpr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let needs_parens = self.precursor > Precursor::expr_precursor(self.expr);
        if needs_parens {
            write!(f, "(")?;
        }
        match self.expr {
            Expr::Num(n, 1) => write!(f, "{n}")?,
            Expr::Num(n, d) => write!(f, "{n}/{d}")?,
            Expr::Float(n) => write!(f, "{n}")?,
            Expr::Sym(s) => write!(f, "{s}")?,
            Expr::Neg(inner) => {
                write!(
                    f,
                    "-{}",
                    DisplayExpr {
                        expr: inner,
                        precursor: Precursor::Atom
                    }
                )?;
            }
            Expr::Add(terms) => {
                for (i, term) in terms.iter().enumerate() {
                    if i == 0 {
                        write!(
                            f,
                            "{}",
                            DisplayExpr {
                                expr: term,
                                precursor: Precursor::Add
                            }
                        )?;
                    } else {
                        // Check if the term is a negation so we can print `- x` instead of `+ -x`
                        match term {
                            Expr::Neg(inner) => {
                                write!(
                                    f,
                                    " - {}",
                                    DisplayExpr {
                                        expr: inner,
                                        precursor: Precursor::Mul
                                    }
                                )?;
                            }
                            Expr::Mul(factors)
                                if !factors.is_empty()
                                    && matches!(factors[0], Expr::Num(n, 1) if n < 0) =>
                            {
                                // e.g. Mul([-3, x])  → " - 3*x"
                                if let Expr::Num(n, 1) = &factors[0] {
                                    let abs_num = Expr::Num(-n, 1);
                                    let mut rest = factors[1..].to_vec();
                                    let pos_term = if rest.len() == 1 {
                                        rest.remove(0)
                                    } else {
                                        Expr::Mul(rest)
                                    };
                                    write!(
                                        f,
                                        " - {}",
                                        DisplayExpr {
                                            expr: &Expr::Mul(vec![abs_num, pos_term]),
                                            precursor: Precursor::Add,
                                        }
                                    )?;
                                }
                            }
                            other => {
                                write!(
                                    f,
                                    " + {}",
                                    DisplayExpr {
                                        expr: other,
                                        precursor: Precursor::Add
                                    }
                                )?;
                            }
                        }
                    }
                }
            }
            Expr::Mul(factors) => {
                // Check if first factor is -1: print as negation
                if factors.len() == 2
                    && let Expr::Num(-1, 1) = &factors[0]
                {
                    write!(
                        f,
                        "-{}",
                        DisplayExpr {
                            expr: &factors[1],
                            precursor: Precursor::Atom
                        }
                    )?;
                    if needs_parens {
                        write!(f, ")")?;
                    }
                    return Ok(());
                }
                for (i, factor) in factors.iter().enumerate() {
                    if i > 0 {
                        write!(f, "*")?;
                    }
                    write!(
                        f,
                        "{}",
                        DisplayExpr {
                            expr: factor,
                            precursor: Precursor::Mul
                        }
                    )?;
                }
            }
            Expr::Factorial(base) => {
                write!(
                    f,
                    "{}!",
                    DisplayExpr {
                        expr: base,
                        precursor: Precursor::Atom
                    }
                )?;
            }
            Expr::Modulo(a, b) => {
                write!(
                    f,
                    "{} % {}",
                    DisplayExpr {
                        expr: a,
                        precursor: Precursor::Mul
                    },
                    DisplayExpr {
                        expr: b,
                        precursor: Precursor::Mul
                    }
                )?;
            }
            Expr::Pow(base, exp) => {
                write!(
                    f,
                    "{}^{}",
                    DisplayExpr {
                        expr: base,
                        precursor: Precursor::Pow
                    },
                    DisplayExpr {
                        expr: exp,
                        precursor: Precursor::Pow
                    }
                )?;
            }
            Expr::Ln(inner) => {
                write!(
                    f,
                    "ln({})",
                    DisplayExpr {
                        expr: inner,
                        precursor: Precursor::None
                    }
                )?;
            }
            Expr::Sin(inner) => {
                write!(
                    f,
                    "sin({})",
                    DisplayExpr {
                        expr: inner,
                        precursor: Precursor::None
                    }
                )?;
            }
            Expr::Cos(inner) => {
                write!(
                    f,
                    "cos({})",
                    DisplayExpr {
                        expr: inner,
                        precursor: Precursor::None
                    }
                )?;
            }
        }
        if needs_parens {
            write!(f, ")")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::super::{expr::Expr, parser::tests::parse};

    #[test]
    fn display_simple() {
        let e = parse("x^2 + 2*x + 1").unwrap();
        let s = format!("{e}");
        assert!(!s.is_empty());
    }

    #[test]
    fn display_fraction() {
        let e = Expr::rat(3, 4);
        assert_eq!(format!("{e}"), "3/4");
    }
}
