use crate::decimal::DecimalNumber;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    PosNum(DecimalNumber), //Positive number literal

    //Constants
    EulersNum, //Euler's number
    Pi,        //pi

    //Measurement
    PlusMinus, //'±', used as a separator in measurements

    //Operators
    Add,   //'+'
    Minus, //'-'  NOTE: could be prefix 'minus' or the infix subtraction operator
    Mul,   //'*'
    Div,   //'/'

    //Grouping
    LeftParen,  // '('
    RightParen, // ')'

    //End-of-file
    Eof,
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::PosNum(d) => write!(f, "{}", d),
            Token::EulersNum => write!(f, "e"),
            Token::Pi => write!(f, "π"),
            Token::PlusMinus => write!(f, "±"),
            Token::Add => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Eof => write!(f, "EOF"), //useful for debugging
        }
    }
}