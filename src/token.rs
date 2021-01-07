use crate::decimal::DecimalNumber;
use crate::measurement::Measurement;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    PosNum(DecimalNumber), //Positive number literal
    Msmnt(Measurement),    //Measurement literal

    //Constants
    EulersNum, //Euler's number
    Pi,        //pi

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
