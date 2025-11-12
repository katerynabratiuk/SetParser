#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(String),
    Empty,
    SetLiteral(Vec<i32>),            // {1,2,3}
    Range(i32, i32),                 // {a..b} inclusive
    Complement(Box<Expr>),           // A'
    Union(Box<Expr>, Box<Expr>),     // A ∪ B
    Intersect(Box<Expr>, Box<Expr>), // A ∩ B
    Diff(Box<Expr>, Box<Expr>),      // A \ B
    SymDiff(Box<Expr>, Box<Expr>),   // A △ B
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Print(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
