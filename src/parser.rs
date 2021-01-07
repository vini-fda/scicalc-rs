use std::fmt;
use crate::{lexer::Lexer, measurement::Measurement, value::Value};
use crate::token::Token;
use std::panic;

///An expression, stored as a tree structure
///
///Look into "S-expressions" to learn more
///
///Reference: https://en.wikipedia.org/wiki/S-expression
#[derive(Debug)]
enum S {
    Atom(Token), //A single token
    Group(Token, Vec<S>) //An operator and a list of tokens
}


impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Group(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

fn expr(text: &str) -> S {
    let mut lexer = Lexer::new(text);
    expr_bp(&mut lexer, 0)
}

///Parses the expressions using Pratt's method(TDOP).
fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S {
    let first_token = lexer.next();
    let mut lhs = match first_token {
        Token::PosNum(_) | Token::EulersNum | Token::Pi => {
            S::Atom(first_token)
        },
        Token::LeftParen => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::RightParen);
            lhs
        },
        Token::Minus => {
            let ((), r_bp) = prefix_binding_power(&first_token);
            let rhs = expr_bp(lexer, r_bp);
            S::Group(first_token, vec![rhs])
        },
        t => panic!("bad token(lhs): {:?}", t),
    };
    
    loop {
        let token = lexer.peek();
        let op = match token {
            Token::Eof => break,
            Token::Add | Token::Minus | Token::Mul | Token::Div |
            Token::RightParen | Token::PlusMinus => token,
            Token::LeftParen => panic!("Excess left parenthesis \'(\'"),
            t => panic!("bad token(rhs): {:?}", t),
        };
        if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
            if l_bp < min_bp {
                break;
            }
    
            lexer.next();
            let rhs = expr_bp(lexer, r_bp);
    
            lhs = S::Group(op, vec![lhs, rhs]);
        } else {
            //Stop parsing
            break;
        }
    }

    lhs
}

fn prefix_binding_power(op: &Token) -> ((), u8) { 
    match op {
        Token::Minus => ((), 9),
        _ => panic!("bad operator: {:?} (is not a prefix operator)", op),
    }
}

///Optionally returns the binding power of an infix operator.
///
///This is at the core of Pratt's method for parsing, because
///it totally orders the precedence of each operator.
///
///If the operator is not valid, returns None.
fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
    let res = match op {
        Token::Add | Token::Minus => (1, 2),
        Token::Mul | Token::Div => (3, 4),
        Token::PlusMinus => (7,8),
        _ => return None,
    };
    Some(res)
}


fn eval_expr(expression: &S) -> Value {
    match expression {
        S::Atom(token) => {
            match token {
                Token::PosNum(x) => Value::PosNumber(x.as_float()),
                Token::EulersNum => Value::Number(std::f64::consts::E),
                Token::Pi => Value::Number(std::f64::consts::PI),
                _ => panic!("bad token(eval atom): {:?}", token)
            }
        },
        S::Group(op, sub_expressions) => {
            match op {
                Token::Add => {
                    if sub_expressions.len() != 2 {
                        panic!("bad sub-expressions: {:?}, addition ('+') operator is binary.", sub_expressions)
                    } else {
                        let lhs = eval_expr(&sub_expressions[0]);
                        let rhs = eval_expr(&sub_expressions[1]);
                        lhs + rhs
                    }
                },
                Token::Minus => {
                    if sub_expressions.len() == 1 {
                        //Unary minus operator
                        - eval_expr(&sub_expressions[0])
                    } else if sub_expressions.len() != 2 {
                        panic!("bad sub-expressions: {:?}, subtraction ('-') operator is binary.", sub_expressions)
                    } else {
                        let lhs = eval_expr(&sub_expressions[0]);
                        let rhs = eval_expr(&sub_expressions[1]);
                        lhs - rhs
                    }
                },
                Token::Mul => {
                    if sub_expressions.len() != 2 {
                        panic!("bad sub-expressions: {:?}, multiplication ('*') operator is binary.", sub_expressions)
                    } else {
                        let lhs = eval_expr(&sub_expressions[0]);
                        let rhs = eval_expr(&sub_expressions[1]);
                        lhs * rhs
                    }
                },
                Token::Div => {
                    if sub_expressions.len() != 2 {
                        panic!("bad sub-expressions: {:?}, division ('/') operator is binary.", sub_expressions)
                    } else {
                        let lhs = eval_expr(&sub_expressions[0]);
                        let rhs = eval_expr(&sub_expressions[1]);
                        lhs / rhs
                    }
                },
                Token::PlusMinus => {
                    if sub_expressions.len() != 2 {
                        panic!("bad sub-expressions: {:?}, plus-minus ('±') operator is binary.", sub_expressions)
                    } else {
                        let lhs = eval_expr(&sub_expressions[0]);
                        let rhs = eval_expr(&sub_expressions[1]);
                        let x = match lhs {
                            Value::Number(m) | Value::PosNumber(m) => m,
                            _ => panic!("left-hand side is not a number! lhs: {:?}", sub_expressions[0])
                        };
                        let y = match rhs {
                            Value::PosNumber(m) => m,
                            _ => panic!("right-hand side is not a positive number! rhs: {:?}", sub_expressions[1])
                        };
                        Value::Measurement(Measurement::new(x, y))
                    }
                },
                _ => todo!()
            }
        }
    }
}

pub fn eval(input: &str) -> Value {
    let s = expr(input);
    eval_expr(&s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        let s = expr("1 + 2 * 3");
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
        let s = expr("--1 * 2");
        assert_eq!(s.to_string(), "(* (- (- 1)) 2)");
        let s = expr("(((0)))");
        assert_eq!(s.to_string(), "0");
        let s = expr("1 ± 2 * 3");
        assert_eq!(s.to_string(), "(* (± 1 2) 3)");
    }
    #[test]
    fn test_negative() {
        let s = expr("-1.0 ± 2.0");
        assert_eq!(s.to_string(), "(± (- 1.0) 2.0)");
    }
    #[test]
    fn test_eval_simple() {
        let s = expr("1 + 2");
        assert_eq!(s.to_string(), "(+ 1 2)");
        let val = eval_expr(&s);
        match val {
            Value::PosNumber(x) => assert_eq!(x, 3.0),
            _ => panic!("Error")
        }
    }
    #[test]
    fn test_eval_measurement() {
        let s = expr("1.0 ± 2.0");
        assert_eq!(s.to_string(), "(± 1.0 2.0)");
        let val = eval_expr(&s);
        match val {
            Value::Measurement(m) => assert_eq!(m, Measurement::new(1.0, 2.0)),
            _ => panic!("Error")
        }
    }
    #[test]
    fn test_eval_measurement_add() {
        let s = expr("1.0 ± 0.01 + 1.7 ± 0.02");
        assert_eq!(s.to_string(), "(+ (± 1.0 0.01) (± 1.7 0.02))");
        let val = eval_expr(&s);
        match val {
            
            Value::Measurement(m) => {
                let actual_value = Measurement::new(1.0, 0.01) + Measurement::new(1.7, 0.02);
                assert_eq!(m,  actual_value)
            },
            _ => panic!("Error")
        }
    }

    #[test]
    fn test_eval_measurement_div() {
        let s = expr("1.0 ± 0.01 / 1.7 ± 0.02");
        assert_eq!(s.to_string(), "(/ (± 1.0 0.01) (± 1.7 0.02))");
        let val = eval_expr(&s);
        match val {
            
            Value::Measurement(m) => {
                let actual_value = Measurement::new(1.0, 0.01) / Measurement::new(1.7, 0.02);
                assert_eq!(m,  actual_value)
            },
            _ => panic!("Error")
        }
    }
    #[test]
    fn test_eval_measurement_mul() {
        let s = expr("1.0 ± 0.01 * 1.7 ± 0.02");
        assert_eq!(s.to_string(), "(* (± 1.0 0.01) (± 1.7 0.02))");
        let val = eval_expr(&s);
        match val {
            
            Value::Measurement(m) => {
                let actual_value = Measurement::new(1.0, 0.01) * Measurement::new(1.7, 0.02);
                assert_eq!(m,  actual_value)
            },
            _ => panic!("Error")
        }
    }
    #[test]
    fn test_valid_parenthesis() {
        let s = expr("(-1.0) ± 2.0");
        assert_eq!(s.to_string(), "(± (- 1.0) 2.0)")
    }
    #[test]
    #[should_panic]
    fn test_wrong_parenthesis() {
        expr("-1.0 (± 2.0");
    }
}