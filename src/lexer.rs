use crate::decimal::DecimalNumber;
use std::{iter::Peekable, str::Chars};

use crate::token::Token;

struct Scanner<'a> {
    characters: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'_> {
    fn new(input: &'a str) -> Scanner<'a> {
        let characters = input.chars().peekable();

        Scanner { characters }
    }

    fn next(&mut self) -> Option<char> {
        self.characters.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.characters.peek().copied()
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut scanner = Scanner::new(input);
        let mut opt_c: Option<char>;
        let mut c: char;
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            opt_c = scanner.peek();

            match opt_c {
                Some(val) => {
                    c = val;
                    scanner.next();
                }
                None => {
                    tokens.push(Token::Eof); //EOF - termination point
                    break;
                }
            };

            let opt_token: Option<Token> = match c {
                '+' => {
                    let opt_next_c = scanner.peek();
                    match opt_next_c {
                        Some(val) => {
                            //Detect the digraph '+-' as '±'
                            if val == '-' {
                                scanner.next();
                                Some(Token::PlusMinus)
                            } else {
                                Some(Token::Add)
                            }
                        }
                        None => Some(Token::Add),
                    }
                }
                '-' => Some(Token::Minus),
                '*' => Some(Token::Mul),
                '/' => Some(Token::Div),
                '^' => Some(Token::Caret),
                '±' => Some(Token::PlusMinus),
                'e' => Some(Token::EulersNum),
                'π' => Some(Token::Pi),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '0'..='9' => Some(Lexer::parse_number(c, false, &mut scanner)),
                '.' => Some(Lexer::parse_number(c, true, &mut scanner)),
                ' ' | '\t' | '\n' => continue, //whitespace
                _ => panic!("Unexpected character: \'{}\'", c),
            };
            match opt_token {
                Some(t) => tokens.push(t),
                None => {}
            }
        }

        tokens.reverse();

        Lexer { tokens }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::Eof)
    }

    fn parse_number(init_c: char, mut found_period: bool, scanner: &mut Scanner) -> Token {
        let mut number_str = String::from("");
        let mut opt_c: Option<char>;
        let mut c: char;

        //Add the initial digit (as a character) to the string
        number_str.push(init_c);

        //Incrementally read and build the numeric string
        //as long as there are valid characters
        loop {
            opt_c = scanner.peek();

            match opt_c {
                Some(val) => {
                    c = val;
                }
                None => break, //STOP: reached EOF
            };

            match c {
                '0'..='9' => {
                    number_str.push(c);
                }
                '.' => {
                    if found_period {
                        //STOP
                        break;
                    } else {
                        found_period = true;
                        number_str.push(c);
                    }
                }
                _ => break, //STOP
            }

            scanner.next();
        }

        match number_str.chars().last() {
            Some(val) => {
                if val == '.' {
                    panic!("Error: numeric literal cannot end in a period. Problematic literal: \"{}\"", number_str);
                }
            }
            None => (),
        };
        let number = DecimalNumber::new(number_str.as_str());

        Token::PosNum(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    fn num_eq(_x: &str, _y: Token) {
        let x = DecimalNumber::new(_x);
        let y = match _y {
            Token::PosNum(val) => val,
            _ => DecimalNumber::new("0"),
        };

        assert_eq!(x, y)
    }
    #[test]
    fn test_simple_number() {
        let mut lex = Lexer::new("12");
        let token = lex.next();
        num_eq("12", token);
        assert_eq!(Token::Eof, lex.next());
    }

    #[test]
    fn test_simple_addition() {
        let mut lex = Lexer::new("2 + 3");
        num_eq("2", lex.next());
        assert_eq!(Token::Add, lex.next());
        num_eq("3", lex.next());
        assert_eq!(Token::Eof, lex.next());
    }
    #[test]
    fn test_simple_plus_minus() {
        let mut lex = Lexer::new("2.3 ± 3.3");
        num_eq("2.3", lex.next());
        assert_eq!(Token::PlusMinus, lex.next());
        num_eq("3.3", lex.next());
        assert_eq!(Token::Eof, lex.next());
    }

    #[test]
    fn test_digraph_plus_minus() {
        let mut lex = Lexer::new("2.3 +- 3.3");
        num_eq("2.3", lex.next());
        assert_eq!(Token::PlusMinus, lex.next());
        num_eq("3.3", lex.next());
        assert_eq!(Token::Eof, lex.next());
    }

    #[test]
    fn test_parenthesis() {
        let mut lex = Lexer::new("(2 + 3) - 5");
        assert_eq!(Token::LeftParen, lex.next());
        num_eq("2", lex.next());
        assert_eq!(Token::Add, lex.next());
        num_eq("3", lex.next());
        assert_eq!(Token::RightParen, lex.next());
        assert_eq!(Token::Minus, lex.next());
        num_eq("5", lex.next());
        assert_eq!(Token::Eof, lex.next());
    }

    #[test]
    fn test_float_1() {
        let mut lex = Lexer::new("13.095");
        num_eq("13.095", lex.next());
        assert_eq!(Token::Eof, lex.next());
    }

    #[test]
    fn test_float_2() {
        let mut lex = Lexer::new("0.095");
        num_eq("0.095", lex.next());
        assert_eq!(Token::Eof, lex.next());
    }
    #[test]
    fn test_float_3() {
        let mut lex = Lexer::new(".095");
        num_eq("0.095", lex.next());
        assert_eq!(Token::Eof, lex.next());
    }

    #[test]
    #[should_panic]
    fn test_float_4() {
        Lexer::new("23.");
    }

    #[test]
    #[should_panic]
    fn test_float_5() {
        Lexer::new(".");
    }
    #[test]
    #[should_panic]
    fn test_float_6() {
        Lexer::new("2 + 5 - 33.");
    }

    #[test]
    fn test_eulers_num() {
        let mut lex = Lexer::new("e");
        assert_eq!(Token::EulersNum, lex.next());
        assert_eq!(Token::Eof, lex.next());
    }

    #[test]
    fn test_pi() {
        let mut lex = Lexer::new("π");
        assert_eq!(Token::Pi, lex.next());
        assert_eq!(Token::Eof, lex.next());
    }

    #[test]
    fn test_eof() {
        let mut lex = Lexer::new("");
        assert_eq!(Token::Eof, lex.next());
    }
}
