use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(Rc<String>),
    Ident(Rc<String>),
    Nil,
}

impl Value {
    pub fn truthy(&self) -> bool {
        match *self {
            Self::Nil => false,
            Self::Bool(false) => false,
            _ => true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Number(v) => write!(f, "{}", v),
            Self::String(ref v) => write!(f, "\"{}\"", v),
            Self::Nil => write!(f, "Nil"),
            Self::Bool(true) => write!(f, "True"),
            Self::Bool(false) => write!(f, "False"),
            Self::Ident(ref ident) => write!(f, "@{}", ident),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(f) => Self::Number(-f),
            v => panic!("Expected Number. Encountered {}", v),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => Self::Number(l + r),
            (Self::Number(l), Self::String(ref r)) => Self::String(Rc::new(format!("{}{}", l, r))),
            (Self::String(ref l), Self::Number(r)) => Self::String(Rc::new(format!("{}{}", l, r))),
            (Self::String(ref l), Self::String(ref r)) => Self::String(Rc::new(format!("{}{}", l, r))),
            (l, r) => panic!("Expected Number. Encountered {} and {}", l, r),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => Self::Number(l - r),
            (l, r) => panic!("Expected Number. Encountered {} and {}", l, r),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => Self::Number(l * r),
            (l, r) => panic!("Expected Number. Encountered {} and {}", l, r),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => Self::Number(l / r),
            (l, r) => panic!("Expected Number. Encountered {} and {}", l, r),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Bool(b) => Self::Bool(!b),
            v => panic!("Expected Bool. Encountered {}", v),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l == r,
            (Self::Bool(l), Self::Bool(r)) => l == r,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l.partial_cmp(r),
            _ => None,
        }
    }
}
