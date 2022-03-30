use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn truthy(&self) -> bool {
        match *self {
            Value::Nil => false,
            Value::Bool(false) => false,
            _ => true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Value::Number(v) => write!(f, "{}", v),
            Value::Nil => write!(f, "Nil"),
            Value::Bool(true) => write!(f, "True"),
            Value::Bool(false) => write!(f, "False"),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(f) => Value::Number(-f),
            v => panic!("Expected Number. Encountered {}", v),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
            (l, r) => panic!("Expected Number. Encountered {} and {}", l, r),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
            (l, r) => panic!("Expected Number. Encountered {} and {}", l, r),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
            (l, r) => panic!("Expected Number. Encountered {} and {}", l, r),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l / r),
            (l, r) => panic!("Expected Number. Encountered {} and {}", l, r),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Value::Bool(!b),
            v => panic!("Expected Bool. Encountered {}", v),
        }
    }
}
