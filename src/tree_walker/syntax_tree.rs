use crate::tree_walker::tokens::{LiteralType, Token};

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Expr {
    Ternary(Box<Ternary>),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
}

impl PrettyPrint for Expr {
    fn pretty_print(&self) -> String {
        use Expr::*;
        match *self {
            Ternary(ref e) => e.pretty_print(),
            Binary(ref e) => e.pretty_print(),
            Unary(ref e) => e.pretty_print(),
            Grouping(ref e) => e.pretty_print(),
            Literal(ref e) => e.pretty_print(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Ternary {
    condition: Expr,
    then: Expr,
    r#else: Expr,
}

impl Ternary {
    pub fn new(cond: Expr, th: Expr, el: Expr) -> Self {
        Ternary {
            condition: cond,
            then: th,
            r#else: el,
        }
    }
}

impl PrettyPrint for Ternary {
    fn pretty_print(&self) -> String {
        format!(
            "(ternary {} ? {} : {})",
            self.condition.pretty_print(),
            self.then.pretty_print(),
            self.r#else.pretty_print(),
        )
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Binary {
    pub fn new(l: Expr, op: Token, r: Expr) -> Self {
        Binary {
            left: l,
            operator: op,
            right: r,
        }
    }
}

impl PrettyPrint for Binary {
    fn pretty_print(&self) -> String {
        format!(
            "({} {} {})",
            self.operator.lexeme,
            self.left.pretty_print(),
            self.right.pretty_print(),
        )
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Unary {
    operator: Token,
    right: Expr,
}

impl Unary {
    pub fn new(op: Token, r: Expr) -> Self {
        Unary {
            operator: op,
            right: r,
        }
    }
}

impl PrettyPrint for Unary {
    fn pretty_print(&self) -> String {
        format!("({} {})", self.operator.lexeme, self.right.pretty_print())
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Grouping {
    expression: Expr,
}

impl Grouping {
    pub fn new(e: Expr) -> Self {
        Grouping { expression: e }
    }
}

impl PrettyPrint for Grouping {
    fn pretty_print(&self) -> String {
        format!("(group {})", self.expression.pretty_print())
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Literal {
    value: Option<LiteralType>,
}

impl Literal {
    pub fn new(v: LiteralType) -> Self {
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
