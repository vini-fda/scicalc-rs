use crate::measurement;
use crate::measurement::Measurement;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
pub enum Value {
    PosNumber(f64),
    Number(f64),
    Measurement(Measurement),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::PosNumber(x) | Value::Number(x) => {
                write!(f, "{}", x)
            }
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
            Value::PosNumber(x) => match _rhs {
                Value::PosNumber(y) => Value::PosNumber(x + y),
                Value::Number(y) => Value::Number(x + y),
                Value::Measurement(y) => Value::Measurement(y + x),
            },
            Value::Number(x) => match _rhs {
                Value::PosNumber(y) => Value::Number(x + y),
                Value::Number(y) => Value::Number(x + y),
                Value::Measurement(y) => Value::Measurement(y + x),
            },
            Value::Measurement(x) => match _rhs {
                Value::PosNumber(y) => Value::Measurement(x + y),
                Value::Number(y) => Value::Measurement(x + y),
                Value::Measurement(y) => Value::Measurement(x + y),
            },
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, _rhs: Value) -> Value {
        match self {
            Value::PosNumber(x) => match _rhs {
                Value::PosNumber(y) => Value::PosNumber(x - y),
                Value::Number(y) => Value::Number(x - y),
                Value::Measurement(y) => Value::Measurement(-y + x),
            },
            Value::Number(x) => match _rhs {
                Value::PosNumber(y) => Value::Number(x - y),
                Value::Number(y) => Value::Number(x - y),
                Value::Measurement(y) => Value::Measurement(-y + x),
            },
            Value::Measurement(x) => match _rhs {
                Value::PosNumber(y) => Value::Measurement(x - y),
                Value::Number(y) => Value::Measurement(x - y),
                Value::Measurement(y) => Value::Measurement(x - y),
            },
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, _rhs: Value) -> Value {
        match self {
            Value::PosNumber(x) => match _rhs {
                Value::PosNumber(y) => Value::PosNumber(x * y),
                Value::Number(y) => Value::Number(x * y),
                Value::Measurement(y) => Value::Measurement(y * x),
            },
            Value::Number(x) => match _rhs {
                Value::PosNumber(y) => Value::Number(x * y),
                Value::Number(y) => Value::Number(x * y),
                Value::Measurement(y) => Value::Measurement(y * x),
            },
            Value::Measurement(x) => match _rhs {
                Value::PosNumber(y) => Value::Measurement(x * y),
                Value::Number(y) => Value::Measurement(x * y),
                Value::Measurement(y) => Value::Measurement(x * y),
            },
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, _rhs: Value) -> Value {
        match self {
            Value::PosNumber(x) => match _rhs {
                Value::PosNumber(y) => Value::PosNumber(x / y),
                Value::Number(y) => Value::Number(x / y),
                Value::Measurement(y) => Value::Measurement(Measurement::new(x, 0.0) / y),
            },
            Value::Number(x) => match _rhs {
                Value::PosNumber(y) => Value::Number(x / y),
                Value::Number(y) => Value::Number(x / y),
                Value::Measurement(y) => Value::Measurement(Measurement::new(x, 0.0) / y),
            },
            Value::Measurement(x) => match _rhs {
                Value::PosNumber(y) => Value::Measurement(x / y),
                Value::Number(y) => Value::Measurement(x / y),
                Value::Measurement(y) => Value::Measurement(x / y),
            },
        }
    }
}

pub fn pow(_lhs: Value, _rhs: Value) -> Value {
    match _lhs {
        Value::PosNumber(x) => match _rhs {
            Value::PosNumber(y) | Value::Number(y) => Value::PosNumber(x.powf(y)),
            Value::Measurement(y) => {
                let r = measurement::pow(Measurement::from(x), y);
                Value::Measurement(r)
            }
        },
        Value::Number(x) => {
            if x < 0.0 {
                match _rhs {
                    Value::PosNumber(y) | Value::Number(y) => {
                        if y.fract() == 0.0 {
                            Value::Number(x.powf(y))
                        } else {
                            panic!("Error! Exponentiation x^y where x < 0.0 and y is non-integer is not allowed!")
                        }
                    },
                    Value::Measurement(_) => panic!("Error! Exponentiation x^y where x < 0.0 and y is a Measurement is not allowed!"),
                }
            } else {
                match _rhs {
                    Value::Number(y) | Value::PosNumber(y) => Value::Number(x.powf(y)),
                    Value::Measurement(y) => {
                        let r = measurement::pow(Measurement::from(x), y);
                        Value::Measurement(r)
                    }
                }
            }
        }
        Value::Measurement(x) => {
            if x.mean - x.sigma < 0.0 {
                panic!("Error! Cannot calculate powers with negative measurements as the base. e.g. (-1.0 ± 0.1)^(2.1)")
            } else {
                match _rhs {
                    Value::PosNumber(y) | Value::Number(y) => {
                        let r = measurement::pow(x, Measurement::from(y));
                        Value::Measurement(r)
                    }
                    Value::Measurement(y) => Value::Measurement(measurement::pow(x, y)),
                }
            }
        }
    }
}
