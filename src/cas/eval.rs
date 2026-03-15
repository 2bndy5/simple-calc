//! Numerical evaluation of symbolic expressions.
//!
//! Given a set of variable bindings (`&[(&str, f64)]`), `eval` recursively
//! folds an `Expr` tree to a single `f64`.
//!
//! Built-in constants:
//!   - `pi` → std::f64::consts::PI
//!   - `e`  → std::f64::consts::E

use super::{error::CasError, expr::Expr};
use std::collections::HashMap;

/// Evaluate an expression numerically.
///
/// # Arguments
/// * `expr` – the expression to evaluate
/// * `env`  – variable bindings, e.g. `&[("x", 2.0), ("y", -1.0)]`
///
/// # Errors
/// Returns `Err(String)` when an unbound symbol is encountered or an
/// undefined operation is performed (e.g. ln of a non-positive number).
pub fn eval(expr: &Expr, env: &[(&str, f64)]) -> Result<f64, CasError> {
    let map: HashMap<&str, f64> = env.iter().cloned().collect();
    eval_inner(expr, &map)
}

pub(crate) fn eval_inner(expr: &Expr, env: &HashMap<&str, f64>) -> Result<f64, CasError> {
    match expr {
        Expr::Num(n, d) => Ok(*n as f64 / *d as f64),
        Expr::Float(inner) => Ok(*inner),
        Expr::Sym(name) => match name.as_str() {
            "pi" => Ok(std::f64::consts::PI),
            "e" => Ok(std::f64::consts::E),
            _ => env
                .get(name.as_str())
                .copied()
                .ok_or_else(|| CasError::UnboundedSymbol(name.clone())),
        },
        Expr::Neg(inner) => Ok(-eval_inner(inner, env)?),
        Expr::Add(terms) => terms
            .iter()
            .map(|t| eval_inner(t, env))
            .try_fold(0.0_f64, |acc, r| r.map(|v| acc + v)),
        Expr::Mul(factors) => factors
            .iter()
            .map(|f| eval_inner(f, env))
            .try_fold(1.0_f64, |acc, r| r.map(|v| acc * v)),
        Expr::Pow(base, exp) => {
            let b = eval_inner(base, env)?;
            let e = eval_inner(exp, env)?;
            Ok(b.powf(e))
        }
        Expr::Ln(inner) => {
            let v = eval_inner(inner, env)?;
            if v <= 0.0 {
                Err(CasError::Undefined(format!(
                    "ln of non-positive value: {v}"
                )))
            } else {
                Ok(v.ln())
            }
        }
        Expr::Sin(inner) => Ok(eval_inner(inner, env)?.sin()),
        Expr::Cos(inner) => Ok(eval_inner(inner, env)?.cos()),
        Expr::Factorial(expr) => {
            let base = eval_inner(expr, env)?;
            if base < 0.0 {
                return Err(CasError::Undefined(format!(
                    "factorial is only defined for non-negative integers, got {base}"
                )));
            }
            if base == 0.0 {
                return Ok(1.0);
            }
            if base.fract() == 0.0 {
                let base = base.floor() as u64;
                let mut result = 1_u64;
                for i in 1..=base {
                    result = result.checked_mul(i).ok_or(CasError::Undefined(format!(
                        "factorial overflow for {base}!"
                    )))?;
                }
                return Ok(result as f64);
            }
            Ok(libm::tgamma(base))
        }
        Expr::Modulo(expr, expr1) => {
            let a = eval_inner(expr, env)?;
            let b = eval_inner(expr1, env)?;
            if b == 0.0 {
                return Err(CasError::Undefined("modulo by zero".to_string()));
            }
            Ok(a % b)
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::super::{expr::Expr, parser::tests::parse};
    use super::eval;

    #[test]
    fn eval_number() {
        assert_eq!(eval(&Expr::rat(3, 2), &[]).unwrap(), 1.5);
    }

    #[test]
    fn eval_pi() {
        let result = eval(&Expr::Sym("pi".to_string()), &[]).unwrap();
        assert!((result - std::f64::consts::PI).abs() < 1e-12);
    }

    #[test]
    fn eval_expression() {
        let e = parse("x^2 + 1").unwrap();
        let result = eval(&e, &[("x", 3.0)]).unwrap();
        assert!((result - 10.0).abs() < 1e-12);
    }

    #[test]
    fn eval_trig() {
        let e = parse("sin(pi)").unwrap();
        let result = eval(&e, &[]).unwrap();
        assert!(result.abs() < 1e-12);
    }

    #[test]
    fn eval_ln_e() {
        let e = parse("ln(e)").unwrap();
        let result = eval(&e, &[]).unwrap();
        assert!((result - 1.0).abs() < 1e-12);
    }

    #[test]
    fn eval_unbound_symbol_error() {
        let e = parse("x + 1").unwrap();
        assert!(eval(&e, &[]).is_err());
    }

    #[test]
    fn eval_ln_negative_error() {
        let e = parse("ln(-1)").unwrap();
        assert!(eval(&e, &[]).is_err());
    }
}
