///Stores the integral/integer and fractional parts of a number
///written in decimal representation as strings.
///
///For example: 3.14159265 has 
///integral = '3' and fractional = '14159265'
#[derive(Debug, Clone, PartialEq)]
pub struct DecimalNumber {
    integral: String,
    fractional: String
}

impl DecimalNumber {
    pub fn new(text: &str) -> DecimalNumber {
        let parts: Vec<&str> = text.split('.').collect();
        if parts.len() == 1 {
            DecimalNumber {
                integral: parts[0].into(),
                fractional: "".into()
            }
        } else {
            DecimalNumber {
                integral: match parts[0] {
                    "" => "0",
                    p => p
                }.into(),
                fractional: parts[1].into()
            }
        }
    }
    pub fn full_number(&self) -> String {
        format!("{}.{}", self.integral, self.fractional)
    }
}


fn sig_figs(number: &str) -> usize {
    let d = DecimalNumber::new(number);
    sig_figs_helper(d)
}

///Does the character-by-character counting
///for the amount of significant figures in a decimal number
fn sig_figs_helper(x: DecimalNumber) -> usize {
    let mut start_count = false;
    let mut counter: usize = 0;
    let mut backtracking = false;
    let mut backtrack_value: usize = 0;

    for c in x.integral.chars() {
        if c != '0' {
            start_count = true;
        }
        if start_count {
            if c == '0' && !backtracking {
                backtrack_value = counter;
                backtracking = true;
            } else if c != '0' {
                backtrack_value += 1;
                backtracking = false;
            }
            counter += 1;
        }
    }

    if x.fractional.chars().count() == 0 {
        return if backtracking { backtrack_value } else { counter };
    }

    for c in x.fractional.chars() {
        if c != '0' {
            start_count = true;
        }
        if start_count {
            counter += 1;
        }
    }

    return counter;
}


#[cfg(test)]
mod tests {
    use super::{DecimalNumber, sig_figs};

    #[test]
    fn create_decimal_number() {
        let d = DecimalNumber::new("1.23");
        assert_eq!("1", d.integral);
        assert_eq!("23", d.fractional);
    }

    #[test]
    fn test_sig_figs() {
        assert_eq!(2, sig_figs("81"));
        assert_eq!(3, sig_figs("81.3"));
        assert_eq!(1, sig_figs("0.3"));
        assert_eq!(2, sig_figs("0.30"));
        assert_eq!(4, sig_figs("0.3000"));
        assert_eq!(1, sig_figs("0.00001"));
        assert_eq!(4, sig_figs("380.0"));
        assert_eq!(3, sig_figs("78800"));
        assert_eq!(6, sig_figs("78800.0"));
    }
}