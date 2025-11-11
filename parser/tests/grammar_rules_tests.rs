use set_parser::{SetLang, Rule, parse_program};
use pest::Parser;

#[test]
fn decl_and_print_stmt() {
    let src = r#"
        let A = {1,2,3};
        print A ∪ {4};
    "#;
    let program = parse_program(src).expect("program should parse");
    assert_eq!(program.stmts.len(), 2);
}

#[test]
fn missing_semicolon_is_error() {
    let src = r#"
        let A = {1,2,3};
        print A ∪ {4}
    "#;
    assert!(parse_program(src).is_err());
}

#[test]
fn comments_and_whitespace() {
    let src = r#"
        // some comment
        let   A={1 , 2 ,3 };   // other comment
        print A ∩ {2}; // last comment
    "#;
    parse_program(src).expect("comments/whitespace should parse");
}

#[test]
fn lexemes_ident_and_int() {
    SetLang::parse(Rule::ident, "A").expect("ident A");
    SetLang::parse(Rule::ident, "Universe").expect("ident Universe");
    SetLang::parse(Rule::int, "0").expect("int 0");
    SetLang::parse(Rule::int, "12345").expect("int 12345");
}

#[test]
fn empty_set_token_and_usage() {
    SetLang::parse(Rule::empty, "∅").expect("empty token");
    let src = r#"
        let D = ∅;
        print D;
    "#;
    parse_program(src).expect("program with ∅ should parse");
}

#[test]
fn set_literal_and_range() {
    SetLang::parse(Rule::set_literal, "{ }").expect("empty literal");
    SetLang::parse(Rule::set_literal, "{1}").expect("one literal");
    SetLang::parse(Rule::set_literal, "{1,2,3}").expect("many literal");
    SetLang::parse(Rule::range, "{1..10}").expect("range up");
    SetLang::parse(Rule::range, "{10..1}").expect("range down");
}

#[test]
fn set_rule_variants() {
    SetLang::parse(Rule::set, "A").expect("set ident");
    SetLang::parse(Rule::set, "∅").expect("set empty");
    SetLang::parse(Rule::set, "{1,2}").expect("set literal");
    SetLang::parse(Rule::set, "{1..3}").expect("set range");
}

#[test]
fn postfix_complements() {
    SetLang::parse(Rule::postfix, "A'").expect("postfix 1");
    SetLang::parse(Rule::postfix, "(A ∪ B)'''").expect("postfix 3");
    let src = r#"
        let universe = {1,2,3,4};
        let A = {1,2};
        print A' ∩ {3,4};
    "#;
    parse_program(src).expect("complements + intersection program should parse");
}

#[test]
fn expr_precedence_examples() {
    let stmts = [
        "print A' ∪ B;",
        "print A ∩ B ∪ C;",
        "print A ∪ B ∩ C;",
        "print (A ∩ B)' △ (A △ B);",
        "print (A' \\ C) △ B;",
    ];
    for s in stmts {
        let src = format!(
            "let universe={{1,2,3}}; let A={{1}}; let B={{2}}; let C={{3}}; {}",
            s
        );
        parse_program(&src).expect(s);
    }
}

#[test]
fn primary_parens() {
    SetLang::parse(Rule::primary, "({1,2} ∪ {3})").expect("primary parens 1");
    SetLang::parse(Rule::primary, "({1} ∩ ({2} ∪ {3}))").expect("primary parens 2");
}

#[test]
fn program_with_range_and_difference() {
    let src = r#"
        let X = {1..5};
        let Y = {3..7};
        print X \ Y;
    "#;
    parse_program(src).expect("ranges + difference should parse");
}

#[test]
fn print_stmt_rule() {
    SetLang::parse(Rule::print_stmt, "print A ∪ B;").expect("print_stmt");
}

#[test]
fn decl_rule() {
    SetLang::parse(Rule::decl, "let A = {1,2,3};").expect("decl");
}

#[test]
fn program_rule() {
    SetLang::parse(Rule::program, "let A={1}; print A;").expect("program");
}

#[test]
fn symmetric_difference_rule() {
    SetLang::parse(Rule::expr, "A △ B").expect("expr with △");
}

#[test]
fn difference_rule() {
    SetLang::parse(Rule::expr, "A \\ B").expect("expr with \\");
}

#[test]
fn intersection_rule() {
    SetLang::parse(Rule::expr, "A ∩ B").expect("expr with ∩");
}

#[test]
fn union_rule() {
    SetLang::parse(Rule::expr, "A ∪ B").expect("expr with ∪");
}

#[test]
fn complement_rule() {
    SetLang::parse(Rule::postfix, "A''").expect("two complements");
}
