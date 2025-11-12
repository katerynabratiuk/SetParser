//! A tiny parser for a small set-algebra DSL.
//!
//! Grammar rules
//!
//! - `WHITESPACE` — internal rule. Matches spaces, tabs, CR and LF and is skipped by the
//!   parser as typical whitespace.
//! - `COMMENT` — internal rule for single-line comments starting with `//`.
//!
//! Token rules:
//! - `ident` — identifier (one or more ASCII letters). Used for set names and variables.
//! - `int` — integer literal (one or more ASCII digits).
//! - `empty` — the empty set token `∅`.
//!
//! Top-level constructs:
//! - `program` — the entire input: start-of-input, zero or more statements, end-of-input.
//! - `stmt` — a single statement; either a `decl` (let declaration) or a `print_stmt`.
//! - `decl` — a `let` declaration: `let <ident> = <expr> ;`.
//! - `print_stmt` — a print statement: `print <expr> ;`.
//!
//! Set operators (tokens used inside expressions):
//! - `complement` — postfix complement operator `'` (like `A'`).
//! - `union` — union operator `∪` (binary).
//! - `intersection` — intersection operator `∩` (binary).
//! - `difference` — difference operator `\` (binary).
//! - `symmetric_difference` — symmetric difference operator `△` (binary).
//!
//! Expressions and precedence:
//! - `expr` — top-level expression nonterminal. Uses `add` as the entry point for precedence.
//! - `add` — handles `union` and `symmetric_difference` at the lowest binary-precedence level.
//! - `mul` — handles `intersection` and `difference` above `add` precedence.
//! - `postfix` — primary expression followed by zero or more `complement` (postfix `'`).
//! - `primary` — either a `set` or a parenthesized sub-expression `( expr )`.
//!
//! Set forms:
//! - `set` — can be an identifier, the empty-set token, a `set_literal`, or a numeric `range`.
//! - `set_literal` — a literal set like `{1,2,3}` (possibly empty: `{}`).
//! - `range` — an inclusive integer range like `{1..10}` representing integers from `1` to `10`.
//!
//! Example:
//! --------
//! ```text
//! let universe = {1,2,3,4,5};
//! let A = {1,2};
//! let B = A' \ {2};
//! print A ∪ B;
//! ```
//!
//! See `src/grammar.pest` for grammar.

pub mod ast;
pub mod eval;
pub mod parse;

pub use parse::{Rule, SetLang, parse_program};
