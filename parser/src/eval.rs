use std::collections::{BTreeSet, HashMap};
use thiserror::Error;
use crate::ast::{Expr, Stmt, Program};

pub type Set = BTreeSet<i32>;

#[derive(Default)]
pub struct Env {
    pub vars: HashMap<String, Set>,
    pub universe: Option<Set>,
}

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("Undefined identifier: {0}")]
    Undefined(String),
    #[error("Universe is required for complement)")]
    NoUniverse,
}

impl Env {
    pub fn eval_program(&mut self, p: &Program) -> Result<Vec<Set>, EvalError> {
        let mut prints = Vec::new();
        for s in &p.stmts {
            match s {
                Stmt::Let { name, value } => {
                    let set = self.eval_expr(value)?;
                    if name == "universe" { self.universe = Some(set.clone()); }
                    self.vars.insert(name.clone(), set);
                }
                Stmt::Print(e) => {
                    prints.push(self.eval_expr(e)?);
                }
            }
        }
        Ok(prints)
    }

    pub fn eval_expr(&self, e: &Expr) -> Result<Set, EvalError> {
        use Expr::*;
        Ok(match e {
            Ident(s) => self.vars.get(s).cloned().ok_or_else(|| EvalError::Undefined(s.clone()))?,
            Empty => Set::new(),
            SetLiteral(vs) => vs.iter().copied().collect(),
            Range(a,b) => {
                let (lo, hi) = if a <= b { (*a, *b) } else { (*b, *a) };
                (lo..=hi).collect()
            }
            Complement(inner) => {
                let uni = self.universe.as_ref().ok_or(EvalError::NoUniverse)?;
                uni.difference(&self.eval_expr(inner)?).copied().collect()
            }
            Union(a,b) => {
                let (a,b) = (self.eval_expr(a)?, self.eval_expr(b)?);
                a.union(&b).copied().collect()
            }
            Intersect(a,b) => {
                let (a,b) = (self.eval_expr(a)?, self.eval_expr(b)?);
                a.intersection(&b).copied().collect()
            }
            Diff(a,b) => {
                let (a,b) = (self.eval_expr(a)?, self.eval_expr(b)?);
                a.difference(&b).copied().collect()
            }
            SymDiff(a,b) => {
                let (a,b) = (self.eval_expr(a)?, self.eval_expr(b)?);
                a.symmetric_difference(&b).copied().collect()
            }
        })
    }

}
