use crate::measurement::Measurement;
use lazy_static::lazy_static;
use regex::Regex;

fn is_num(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\-?)(\d+)(\.\d+)?|(\.\d+)").unwrap();
    }
    RE.is_match(&text)
}

///Checks for measurements of the following types:
///
/// (x +- y), (x ± y), (x)
///
///where x and y are numeric literals(e.g. 2.33, 5, .67)
///
///Note that 'y' cannot be a negative number, whereas x can
///
///(so '-2.33 ± 2.0' is valid, but '2.0 ± -1.0' is not)
fn is_measurement(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<mean>(\-?)(\d+)(\.\d+)?|(\-?)(\.\d+))\s*(\+\-|±)\s*(?P<sigma>(\d+)(\.\d+)?|(\.\d+))").unwrap();
    }
    RE.is_match(&text)
}

fn parse_measurement(text: &str) -> Result<Measurement, &str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<mean>(\-?)(\d+)(\.\d+)?|(\-?)(\.\d+))(\s*(\+\-|±)\s*(?P<sigma>(\d+)(\.\d+)?|(\.\d+)))?").unwrap();
    }
    if !RE.is_match(&text) {
        Err("The string does not contain a measurement literal. Maybe there was a syntax error?")
    } else {
        //Get all capture groups from the string with the given regular expression
        let captures = RE.captures(text).unwrap();
        //mean is required
        let mean: &str = captures.name("mean").unwrap().as_str();
        let mean: f64 = mean.parse::<f64>().unwrap();
        //sigma is optional
        //when a number is given without sigma(e.g. '70.5'), sigma is implicitly
        //given the value of 0.0 (in the example, '70.5 +- 0.0')
        let sigma: f64 = match captures.name("sigma") {
            Some(text) => text.as_str().parse::<f64>().unwrap(),
            None => 0.0,
        };

        Ok(Measurement::new(mean, sigma))
    }
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
    #[test]
    fn unicode_plus_minus_measurement() {
        assert!(is_measurement("23.333 ± 2.0"));
        assert!(is_measurement("23.333 ±2.0"));
        assert!(is_measurement("23.333± 2.0"));
        assert!(is_measurement("23.333±2.0"));
    }

    #[test]
    fn unicode_plus_minus_disallow_negative_uncertainty_measurement() {
        //Negative error/sigma should NOT be parsed
        //even with unicode ±
        assert!(!is_measurement("23.333 ± -2.0"));
        assert!(!is_measurement("23.333 ±-2.0"));
        assert!(!is_measurement("23.333± -2.0"));
        assert!(!is_measurement("23.333±-2.0"));
    }

    //Test parsing
    #[test]
    fn parse_simple_measurement() {
        let parse_result = parse_measurement("1.0 +- 0.01");
        let parse_result = match parse_result {
            Ok(result) => result,
            Err(error_msg) => panic!("Error when parsing the number! {}", error_msg),
        };
        let actual_value = Measurement::new(1.0, 0.01);
        assert_eq!(actual_value, parse_result);
    }
    #[test]
    fn parse_negative_measurement() {
        let parse_result = parse_measurement("-3.33 +- 0.01");
        let parse_result = match parse_result {
            Ok(result) => result,
            Err(error_msg) => panic!("Error when parsing the number! {}", error_msg),
        };
        let actual_value = Measurement::new(-3.33, 0.01);
        assert_eq!(actual_value, parse_result);
    }
    #[test]
    fn parse_measurement_without_sigma() {
        let parse_result = parse_measurement("70.5");
        let parse_result = match parse_result {
            Ok(result) => result,
            Err(error_msg) => panic!("Error when parsing the number! {}", error_msg),
        };
        let actual_value = Measurement::new(70.5, 0.0);
        assert_eq!(actual_value, parse_result);
    }
}
