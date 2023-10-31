use crate::tree_walker::tokens::{LiteralType, Token};

use super::errors::RuntimeError;
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

#[derive(Debug)]
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

pub fn interpret(expr: Expr) {
    match expr.eval() {
        Ok(value) => println!("{:?}", value),
        Err(e) => e.report(),
    }
}

pub trait Eval {
    fn eval(&self) -> Result<Value, RuntimeError>;
}

impl Eval for Expr {
    fn eval(&self) -> Result<Value, RuntimeError> {
        use Expr::*;
        match *self {
            Ternary(ref t) => t.eval(),
            Binary(ref b) => b.eval(),
            Unary(ref u) => u.eval(),
            Grouping(ref g) => g.eval(),
            Literal(ref l) => l.eval(),
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
    fn eval(&self) -> Result<Value, RuntimeError> {
        let condition = self.condition.eval()?;

        match condition {
            Value::Boolean(b) => {
                if b {
                    return self.then.eval();
                }
                self.r#else.eval()
            }
            _ => Err(RuntimeError::new(
                Token {
                    r#type: TokenType::Question,
                    lexeme: "?".to_string(),
                    literal: None,
                    line: 0,
                },
                "We shouldn't be here... ternary condition didn't returned a boolean.".to_string(),
            )),
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
    fn eval(&self) -> Result<Value, RuntimeError> {
        let left = self.left.eval()?;
        let right = self.right.eval()?;

        if !Value::variant_eq(&left, &right) {
            return Err(RuntimeError::new(
                self.operator.to_owned(),
                "Types don't match in binary expression.".to_string(),
            ));
        }

        if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
            match self.operator.r#type {
                TokenType::Plus => return Ok(Value::Number(l + r)),
                TokenType::Minus => return Ok(Value::Number(l - r)),
                TokenType::Slash => return Ok(Value::Number(l / r)),
                TokenType::Star => return Ok(Value::Number(l * r)),
                TokenType::Greater => return Ok(Value::Boolean(l > r)),
                TokenType::GreaterEqual => return Ok(Value::Boolean(l >= r)),
                TokenType::Less => return Ok(Value::Boolean(l < r)),
                TokenType::LessEqual => return Ok(Value::Boolean(l <= r)),
                TokenType::BangEqual => return Ok(Value::Boolean(!(l == r))),
                TokenType::EqualEqual => return Ok(Value::Boolean(l == r)),
                _ => {
                    return Err(RuntimeError::new(
                        self.operator.to_owned(),
                        "Invalid binary expression operator.".to_string(),
                    ));
                }
            }
        }

        if let (Value::String(l), Value::String(r)) = (&left, &right) {
            match self.operator.r#type {
                TokenType::Plus => return Ok(Value::String(l.to_owned() + r)),
                _ => {
                    return Err(RuntimeError::new(
                        self.operator.to_owned(),
                        "Invalid binary expression operator.".to_string(),
                    ));
                }
            }
        }

        Err(RuntimeError::new(
            self.operator.to_owned(),
            "Invalid binary expression operator.".to_string(),
        ))
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
    fn eval(&self) -> Result<Value, RuntimeError> {
        let right = self.right.eval()?;

        match self.operator.r#type {
            TokenType::Bang => Ok(Value::Boolean(!right.is_truthy())),
            TokenType::Minus => match right {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::new(
                    self.operator.to_owned(),
                    "Minus should only be used with the number type.".to_string(),
                )),
            },
            _ => Err(RuntimeError::new(
                self.operator.to_owned(),
                "Invalid operator in unary expression.".to_string(),
            )),
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
    fn eval(&self) -> Result<Value, RuntimeError> {
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
    fn eval(&self) -> Result<Value, RuntimeError> {
        Ok(match self.value.to_owned().unwrap_or(LiteralType::Nil) {
            LiteralType::Number(v) => Value::Number(v),
            LiteralType::String(v) => Value::String(v),
            LiteralType::Bool(v) => Value::Boolean(v),
            LiteralType::Nil => Value::Nil,
        })
    }
}
