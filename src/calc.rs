use std::str::FromStr;

use bevy::prelude::*;

use crate::cas::{ExprBuilder, Operand, Token, error::CasError, eval};

#[derive(Debug, Resource, Default)]
pub struct Calc {
    pub operand: Option<Operand>,
    pub expression: ExprBuilder,
}

impl Calc {
    pub fn display_operand(&self) -> String {
        self.operand
            .as_ref()
            .map(|o| o.to_string())
            .unwrap_or_default()
    }

    pub fn push_operand(&mut self, val: &str) {
        if self
            .expression
            .last()
            .map(|t| matches!(t, Token::Eof))
            .is_some_and(|b| b)
        {
            self.expression.clear();
            if val != "-" {
                self.operand = None;
            }
        }
        if let Some(operand) = &mut self.operand {
            operand.push_str(val);
        } else {
            let mut operand = Operand::default();
            operand.push_str(val);
            self.operand = Some(operand);
        }
    }

    pub fn push_operator(&mut self, op: &str) {
        if let Some(operand) = self.operand.take() {
            if self
                .expression
                .last()
                .map(|t| matches!(t, Token::Eof))
                .is_some_and(|b| b)
            {
                self.expression.clear();
            }
            self.expression.push(operand.get_token());
        }
        if op != "("
            && self
                .expression
                .last()
                .map(|t| {
                    matches!(
                        t,
                        Token::Plus
                            | Token::Minus
                            | Token::Star
                            | Token::Slash
                            | Token::Caret
                            | Token::Modulo
                            | Token::Factorial
                    )
                })
                .unwrap_or(false)
        {
            self.expression.pop();
        }
        self.expression
            .push(Token::from_str(op).unwrap_or(Token::Star));
    }

    pub fn pop(&mut self) {
        if let Some(operand) = self.operand.as_mut()
            && !operand.is_empty()
        {
            operand.pop();
        } else {
            self.operand = None;
            self.expression.pop();
        }
    }

    // pub fn clear_all(&mut self) {
    //     if self.operand.take().is_none() {
    //         self.expression.clear();
    //     }
    // }

    pub fn solve(&mut self) -> Result<(), CasError> {
        if self
            .expression
            .last()
            .map(|t| matches!(t, Token::Eof))
            .is_some_and(|b| b)
        {
            // TODO: we need a way to repeat the last operation against the recent result.
            // Just return early if the expression is already solved.
            return Ok(());
        }
        if let Some(operand) = self.operand.take()
            && !operand.is_empty()
        {
            self.expression.push(operand.get_token());
        }
        self.expression.push(Token::Eof);
        let expr = self.expression.build()?;
        let result = eval(&expr, &[])?;
        self.operand = Some(Operand::new(Token::Number(result.to_string())));
        Ok(())
    }
}
