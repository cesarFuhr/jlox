use crate::tree_walker::tokens::{LiteralType, Token};

use super::tokens::TokenType;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Expr {
    Ternary(Box<Ternary>),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
}

pub fn ast_print(expr: Expr) -> String {
    expr.pretty_print()
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
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

pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => b.to_owned(),
            _ => true,
        }
    }

    fn variant_eq(a: &Value, b: &Value) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }
}

pub trait Eval {
    fn eval(&self) -> Value;
}

impl Eval for Expr {
    fn eval(&self) -> Value {
        use Expr::*;
        match *self {
            Literal(ref e) => e.eval(),
            _ => Value::Nil,
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

impl Eval for Ternary {
    fn eval(&self) -> Value {
        let condition = self.condition.eval();

        match condition {
            Value::Boolean(b) => {
                if b {
                    return self.then.eval();
                }
                self.r#else.eval()
            }
            _ => {
                println!("we shouldn't be here... condition didn't returned a boolean");
                Value::Nil
            }
        }
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

impl Eval for Binary {
    fn eval(&self) -> Value {
        let left = self.left.eval();
        let right = self.right.eval();

        if !Value::variant_eq(&left, &right) {
            println!("types don't match in binary expression");
            return Value::Nil;
        }

        if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
            match self.operator.r#type {
                TokenType::Minus => return Value::Number(l - r),
                TokenType::Slash => return Value::Number(l / r),
                TokenType::Star => return Value::Number(l * r),
                TokenType::Greater => return Value::Boolean(l > r),
                TokenType::GreaterEqual => return Value::Boolean(l >= r),
                TokenType::Less => return Value::Boolean(l < r),
                TokenType::LessEqual => return Value::Boolean(l <= r),
                TokenType::BangEqual => return Value::Boolean(!(l == r)),
                TokenType::EqualEqual => return Value::Boolean(l == r),
                _ => {
                    print!("invalid binary expression operator");
                    return Value::Nil;
                }
            }
        }

        if let (Value::String(l), Value::String(r)) = (&left, &right) {
            match self.operator.r#type {
                TokenType::Plus => return Value::String(l.to_owned() + r),
                _ => {
                    print!("invalid binary expression operator");
                    return Value::Nil;
                }
            }
        }

        print!("invalid binary expression");
        Value::Nil
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

impl Eval for Unary {
    fn eval(&self) -> Value {
        let right = self.right.eval();

        match self.operator.r#type {
            TokenType::Bang => Value::Boolean(!right.is_truthy()),
            TokenType::Minus => match right {
                Value::Number(n) => Value::Number(-n),
                _ => panic!("minus should be only used in a number"),
            },
            _ => {
                print!("we shouldn't be here...");
                Value::Nil
            }
        }
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

impl Eval for Grouping {
    fn eval(&self) -> Value {
        self.expression.eval()
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

impl Eval for Literal {
    fn eval(&self) -> Value {
        match self.value.to_owned().unwrap_or(LiteralType::Nil) {
            LiteralType::Number(v) => Value::Number(v),
            LiteralType::String(v) => Value::String(v),
            LiteralType::Bool(v) => Value::Boolean(v),
            LiteralType::Nil => Value::Nil,
        }
    }
}
