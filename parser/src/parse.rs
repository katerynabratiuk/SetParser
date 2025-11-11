use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;
use thiserror::Error;
use crate::ast::{Expr, Stmt, Program};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SetLang;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("pest parse error: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
    #[error("unexpected rule: {0:?}")]
    Unexpected(Rule),
    #[error("expected child in {0:?}")]
    Missing(Rule),
}

pub fn parse_program(src: &str) -> Result<Program, ParseError> {
    let mut pairs = SetLang::parse(Rule::program, src)?;
    let program = pairs.next().ok_or(ParseError::Missing(Rule::program))?;
    let mut stmts = Vec::new();
    for p in program.into_inner() {
        match p.as_rule() {
            Rule::stmt => stmts.push(parse_stmt(p)?),
            Rule::EOI => {},
            _ => {}
        }
    }
    Ok(Program { stmts })
}

fn parse_stmt(pair: Pair<Rule>) -> Result<Stmt, ParseError> {
    let p = single(pair)?;
    match p.as_rule() {
        Rule::decl => {
            let mut it = p.into_inner();
            let name = it.next().ok_or(ParseError::Missing(Rule::ident))?.as_str().to_string();
            let expr = parse_expr(it.next().ok_or(ParseError::Missing(Rule::expr))?)?;
            Ok(Stmt::Let { name, value: expr })
        }
        Rule::print_stmt => {
            let expr = parse_expr(single_of(p, Rule::expr)?)?;
            Ok(Stmt::Print(expr))
        }
        _ => Err(ParseError::Unexpected(p.as_rule()))
    }
}

fn parse_expr(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    match pair.as_rule() {
        Rule::expr => parse_expr(single(pair)?),
        Rule::add  => parse_left_chain(pair, Rule::mul, parse_mul, build_add),
        Rule::mul  => parse_mul(pair),
        Rule::postfix => parse_postfix(pair),
        Rule::primary => parse_primary(pair),
        r => Err(ParseError::Unexpected(r)),
    }
}

fn parse_left_chain(
    pair: Pair<Rule>,
    expect_head: Rule,
    parse_head: fn(Pair<Rule>) -> Result<Expr, ParseError>,
    build: fn(Expr, Pair<Rule>, Expr) -> Expr,
) -> Result<Expr, ParseError> {
    let mut it = pair.into_inner();
    let head = it.next().ok_or(ParseError::Missing(expect_head))?;
    let mut acc = parse_head(head)?;
    while let (Some(op), Some(rhs)) = (it.next(), it.next()) {
        let rhs_expr = parse_head(rhs)?;
        acc = build(acc, op, rhs_expr);
    }
    Ok(acc)
}

fn build_add(acc: Expr, op: Pair<Rule>, rhs: Expr) -> Expr {
    match op.as_rule() {
        Rule::union => Expr::Union(Box::new(acc), Box::new(rhs)),
        Rule::symmetric_difference => Expr::SymDiff(Box::new(acc), Box::new(rhs)),
        _ => unreachable!(),
    }
}

fn build_mul(acc: Expr, op: Pair<Rule>, rhs: Expr) -> Expr {
    match op.as_rule() {
        Rule::intersection => Expr::Intersect(Box::new(acc), Box::new(rhs)),
        Rule::difference   => Expr::Diff(Box::new(acc), Box::new(rhs)),
        _ => unreachable!(),
    }
}

fn parse_postfix(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let mut it = pair.into_inner();
    let primary = it.next().ok_or(ParseError::Missing(Rule::primary))?;
    let mut expr = parse_primary(primary)?;
    for tail in it {
        expr = Expr::Complement(Box::new(expr));
    }
    Ok(expr)
}

fn parse_primary(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let p = single(pair)?;
    match p.as_rule() {
        Rule::set => parse_set(p),
        Rule::expr => parse_expr(p),
        _ => Err(ParseError::Unexpected(p.as_rule()))
    }
}

fn parse_set(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let p = single(pair)?;
    match p.as_rule() {
        Rule::ident => Ok(Expr::Ident(p.as_str().to_string())),
        Rule::empty => Ok(Expr::Empty),
        Rule::set_literal => {
            let mut vals = Vec::new();
            for item in p.into_inner() {
                if item.as_rule() == Rule::int {
                    vals.push(item.as_str().parse::<i32>().unwrap());
                }
            }
            Ok(Expr::SetLiteral(vals))
        }
        Rule::range => {
            let mut it = p.into_inner();
            let a = it.next().unwrap().as_str().parse::<i32>().unwrap();
            let b = it.next().unwrap().as_str().parse::<i32>().unwrap();
            Ok(Expr::Range(a,b))
        }
        _ => Err(ParseError::Unexpected(p.as_rule()))
    }
}

fn parse_mul(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    parse_left_chain(pair, Rule::postfix, parse_postfix, build_mul)
}

fn single(pair: Pair<Rule>) -> Result<Pair<Rule>, ParseError> {
    let rule = pair.as_rule();
    let mut it = pair.into_inner();
    it.next().ok_or(ParseError::Unexpected(rule))
}

fn single_of(pair: Pair<Rule>, want: Rule) -> Result<Pair<Rule>, ParseError> {
    let p = single(pair)?;
    if p.as_rule() == want { Ok(p) } else { Err(ParseError::Unexpected(p.as_rule())) }
}
