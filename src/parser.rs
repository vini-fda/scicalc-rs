use lazy_static::lazy_static;
//use crate::measurement::Measurement;
use regex::Regex;

fn is_num(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\-?)(\d+)(\.\d+)?|(\.\d+)").unwrap();
    }
    RE.is_match(&text)
}

fn is_measurement(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)(\.\d+)?|(\.\d+)").unwrap();
    }
    RE.is_match(&text)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negative_float() {
        assert!(is_num("-23.333"));
    }
    #[test]
    fn simple_float() {
        assert!(is_num("23.333"));
    }
    #[test]
    fn integers() {
        assert!(is_num("23"));
        assert!(is_num("0"));
        assert!(is_num("8921801298"));
    }
    #[test]
    fn negative_integers() {
        assert!(is_num("-23"));
        assert!(is_num("-0"));
        assert!(is_num("-8921801298"));
    }
    #[test]
    fn period_floats() {
        assert!(is_num(".333"));
        assert!(is_num(".07"));
    }
    #[test]
    fn negative_period_floats() {
        assert!(is_num("-.333"));
        assert!(is_num("-.07"));
    }
}

// fn parse_num(text: String) -> Option<Measurement> {

// }