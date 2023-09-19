use crate::tree_walker::tokens::{LiteralType, Token};

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

pub enum Expr {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
}

impl PrettyPrint for Expr {
    fn pretty_print(&self) -> String {
        use Expr::*;
        match *self {
            Binary(ref e) => e.pretty_print(),
            Unary(ref e) => e.pretty_print(),
            Grouping(ref e) => e.pretty_print(),
            Literal(ref e) => e.pretty_print(),
        }
    }
}

pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Binary {
    pub fn new(l: Expr, op: Token, r: Expr) -> Binary {
        Binary {
            left: l,
            operator: op,
            right: r,
        }
    }
}

impl PrettyPrint for Binary {
    fn pretty_print(&self) -> String {
        return format!(
            "({} {} {})",
            self.operator.lexeme,
            self.left.pretty_print(),
            self.right.pretty_print(),
        );
    }
}

pub struct Unary {
    operator: Token,
    right: Expr,
}

impl Unary {
    pub fn new(op: Token, r: Expr) -> Unary {
        Unary {
            operator: op,
            right: r,
        }
    }
}

impl PrettyPrint for Unary {
    fn pretty_print(&self) -> String {
        return format!("({} {})", self.operator.lexeme, self.right.pretty_print());
    }
}

pub struct Grouping {
    expression: Expr,
}

impl Grouping {
    pub fn new(e: Expr) -> Grouping {
        Grouping { expression: e }
    }
}

impl PrettyPrint for Grouping {
    fn pretty_print(&self) -> String {
        return format!("(group {})", self.expression.pretty_print());
    }
}

pub struct Literal {
    value: Option<LiteralType>,
}

impl Literal {
    pub fn new(v: LiteralType) -> Literal {
        Literal { value: Some(v) }
    }
}

impl PrettyPrint for Literal {
    fn pretty_print(&self) -> String {
        match &self.value {
            Some(v) => format!("{}", v),
            None => "Nil".to_string(),
        }
    }
}

pub fn ast_print(expr: Expr) -> String {
    expr.pretty_print()
}
