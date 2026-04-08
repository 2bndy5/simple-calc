//! Core symbolic expression type and basic arithmetic utilities for the CAS.

/// The core symbolic expression type for the CAS.
///
/// Every mathematical object is represented as an `Expr` tree.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A rational number stored as (numerator, denominator) in lowest terms.
    Num(i64, i64),
    /// A floating-point number (used for decimal/scientific notation literals).
    Float(f64),
    /// A named symbol, e.g. `x`, `y`, `pi`, `e`.
    Sym(String),
    /// Addition: a + b + … (n-ary, stored as flat list).
    Add(Vec<Expr>),
    /// Multiplication: a * b * … (n-ary, stored as flat list).
    Mul(Vec<Expr>),
    /// Power: base ^ exponent.
    Pow(Box<Expr>, Box<Expr>),
    /// Negation: -x  (sugar for Mul([-1, x])).
    Neg(Box<Expr>),
    /// Modulo: a % b.
    Modulo(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Convenience constructor for an integer number.
    pub fn num(n: i64) -> Self {
        Expr::Num(n, 1)
    }

    /// Convenience constructor for a fractional number.
    pub fn rat(numerator: i64, denominator: i64) -> Self {
        assert!(denominator != 0, "denominator must be non-zero");
        let common_denominator = gcd(numerator.abs(), denominator.abs());
        let sign = if denominator < 0 { -1 } else { 1 };
        Expr::Num(
            sign * numerator / common_denominator,
            sign * denominator / common_denominator,
        )
    }
}

// ------------------------------------------------------------------ arithmetic
/// Compute the Greatest Common Divisor of two integers using the Euclidean algorithm.
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a == 0 { 1 } else { a }
}

// Operator overloading for ergonomic construction in tests / consumer code.
impl std::ops::Add for Expr {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Expr {
        Expr::Add(vec![self, rhs])
    }
}

impl std::ops::Sub for Expr {
    type Output = Expr;
    fn sub(self, rhs: Expr) -> Expr {
        Expr::Add(vec![self, Expr::Neg(Box::new(rhs))])
    }
}

impl std::ops::Mul for Expr {
    type Output = Expr;
    fn mul(self, rhs: Expr) -> Expr {
        Expr::Mul(vec![self, rhs])
    }
}

impl std::ops::Div for Expr {
    type Output = Expr;
    fn div(self, rhs: Expr) -> Expr {
        Expr::Mul(vec![
            self,
            Expr::Pow(Box::new(rhs), Box::new(Expr::num(-1))),
        ])
    }
}

impl std::ops::Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Expr {
        Expr::Neg(Box::new(self))
    }
}
