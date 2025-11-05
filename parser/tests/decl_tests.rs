use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../src/grammar.pest"]
struct Grammar;

const DECL_RULE:Rule = Rule::decl;

#[test]
fn test_decl_accepts_simple_declaration() {
    assert!(Grammar::parse(DECL_RULE, "let A = {1, 2, 3};").is_ok());
    assert!(Grammar::parse(DECL_RULE, "let A = {1..3};").is_ok());
    assert!(Grammar::parse(DECL_RULE, "let A = A ∩ B;").is_ok());
}

#[test]
fn test_decl_doesnt_accept_declaration_without_semicolon() {
    assert!(Grammar::parse(DECL_RULE, "let A = {1, 2, 3}").is_err());
    assert!(Grammar::parse(DECL_RULE, "let A = {1..3}").is_err());
}

#[test]
fn test_decl_doesnt_accept_declaration_without_let_keyword() {
    assert!(Grammar::parse(DECL_RULE, "A = {1, 2, 3};").is_err());
    assert!(Grammar::parse(DECL_RULE, "A = {1..3};").is_err());
}

#[test]
fn test_decl_accepts_range() {
    assert!(Grammar::parse(DECL_RULE, "let A = {1..3};").is_ok());
    assert!(Grammar::parse(DECL_RULE, "let A = {1..100};").is_ok());
    assert!(Grammar::parse(DECL_RULE, "let A = {1..1};").is_ok());
}


#[test]
fn test_decl_accepts_reversed_range() {
    assert!(Grammar::parse(DECL_RULE, "let A = {5..3};").is_ok());
    assert!(Grammar::parse(DECL_RULE, "let A = {1..0};").is_ok());
}