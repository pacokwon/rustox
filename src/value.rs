use std::{ops::{Neg, Add, Sub, Mul, Div}, fmt::Display};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Nil,
    True,
    False,
}

impl Value {
    pub fn as_number(&self) -> f64 {
        match *self {
            Value::Number(f) => f,
            _ => panic!("Expected Number. Encountered {}", *self),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Value::Number(v) => write!(f, "{}", v),
            Value::Nil => write!(f, "Nil"),
            Value::True => write!(f, "True"),
            Value::False => write!(f, "False"),
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
