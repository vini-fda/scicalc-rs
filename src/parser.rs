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
    //Checks for measurements of type (x +- y), where x and y are numeric literals(e.g. 2.33)
    //Examples are given in the 'tests' section
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((\-?)(\d+)(\.\d+)?|(\-?)(\.\d+))\s*\+\-\s*((\d+)(\.\d+)?|(\.\d+))").unwrap();
    }
    RE.is_match(&text)
}

#[cfg(test)]
mod tests {
    use super::*;
    //---------------------
    //Test numeric literal parsing
    //---------------------
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

    //---------------------
    //Test measurement parsing
    //---------------------
    #[test]
    fn basic_measurement() {
        assert!(is_measurement("23.333 +- 2.0"));
        assert!(is_measurement("23.333 +-2.0"));
        assert!(is_measurement("23.333+- 2.0"));
        assert!(is_measurement("23.333+-2.0"));
    }
    #[test]
    fn negative_sigma_disallowed() {
        //Negative error/sigma should NOT be parsed
        assert!(!is_measurement("23.333 +- -2.0"));
        assert!(!is_measurement("23.333 +--2.0"));
        assert!(!is_measurement("23.333+--2.0"));
        assert!(!is_measurement("23.333+- -2.0"));
    }
    #[test]
    fn negative_mean_measurement() {
        //Negative mean is perfectly OK
        assert!(is_measurement("-23.333 +- 2.0"));
        assert!(is_measurement("-23.333 +-2.0"));
        assert!(is_measurement("-23.333+- 2.0"));
        assert!(is_measurement("-23.333+-2.0"));
    }
}
