use std::fmt;
use crate::lexer::Lexer;
use crate::token::Token;

///An expression, stored as a tree structure
///
///Look into "S-expressions" to learn more
///
///Reference: https://en.wikipedia.org/wiki/S-expression
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
            Token::LeftParen | Token::RightParen | Token::PlusMinus => token,
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
}