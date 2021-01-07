use crate::measurement::Measurement;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
pub enum Value {
    PosNumber(f64),
    Number(f64),
    Measurement(Measurement)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::PosNumber(x) | Value::Number(x) => {
                write!(f, "{}", x)
            },
            Value::Measurement(x) => {
                write!(f, "{}", x)
            }
        }
        
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::PosNumber(x) => Value::PosNumber(-x),
            Value::Number(x) => Value::Number(-x),
            Value::Measurement(x) => Value::Measurement(-x),
        }
    }
}


impl Add for Value {
    type Output = Self;

    fn add(self, _rhs: Value) -> Value {
        match self {
            Value::PosNumber(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::PosNumber(x+y),
                    Value::Number(y) => Value::Number(x+y),
                    Value::Measurement(y) => Value::Measurement(y+x),
                }
            },
            Value::Number(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::Number(x+y),
                    Value::Number(y) => Value::Number(x+y),
                    Value::Measurement(y) => Value::Measurement(y+x),
                }
            },
            Value::Measurement(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::Measurement(x+y),
                    Value::Number(y) => Value::Measurement(x+y),
                    Value::Measurement(y) => Value::Measurement(x+y),
                }
            }

        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, _rhs: Value) -> Value {
        match self {
            Value::PosNumber(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::PosNumber(x-y),
                    Value::Number(y) => Value::Number(x-y),
                    Value::Measurement(y) => Value::Measurement(-y+x),
                }
            },
            Value::Number(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::Number(x-y),
                    Value::Number(y) => Value::Number(x-y),
                    Value::Measurement(y) => Value::Measurement(-y+x),
                }
            },
            Value::Measurement(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::Measurement(x-y),
                    Value::Number(y) => Value::Measurement(x-y),
                    Value::Measurement(y) => Value::Measurement(x-y),
                }
            }

        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, _rhs: Value) -> Value {
        match self {
            Value::PosNumber(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::PosNumber(x*y),
                    Value::Number(y) => Value::Number(x*y),
                    Value::Measurement(y) => Value::Measurement(y*x),
                }
            },
            Value::Number(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::Number(x*y),
                    Value::Number(y) => Value::Number(x*y),
                    Value::Measurement(y) => Value::Measurement(y*x),
                }
            },
            Value::Measurement(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::Measurement(x*y),
                    Value::Number(y) => Value::Measurement(x*y),
                    Value::Measurement(y) => Value::Measurement(x*y),
                }
            }

        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, _rhs: Value) -> Value {
        match self {
            Value::PosNumber(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::PosNumber(x/y),
                    Value::Number(y) => Value::Number(x/y),
                    Value::Measurement(y) => Value::Measurement(Measurement::new(x, 0.0) /y),
                }
            },
            Value::Number(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::Number(x/y),
                    Value::Number(y) => Value::Number(x/y),
                    Value::Measurement(y) => Value::Measurement(Measurement::new(x, 0.0)/y),
                }
            },
            Value::Measurement(x) => {
                match _rhs {
                    Value::PosNumber(y) => Value::Measurement(x/y),
                    Value::Number(y) => Value::Measurement(x/y),
                    Value::Measurement(y) => Value::Measurement(x/y),
                }
            }

        }
    }
}