use float_cmp::{ApproxEq, F64Margin};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/**A Measurement 'x' is written as x = (mean +- sigma)

where 'mean' is the mean value

and 'sigma' is the uncertainty(also called error or standard deviation from the mean)*/
#[derive(Debug, Clone, Copy)]
pub struct Measurement {
    pub mean: f64,  //mean value
    pub sigma: f64, //std deviation, error or uncertainty
}

impl Measurement {
    pub fn new(mean: f64, sigma: f64) -> Measurement {
        Measurement { mean, sigma }
    }
}

impl Neg for Measurement {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            mean: -self.mean,
            sigma: self.sigma,
        }
    }
}

impl Add for Measurement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            mean: self.mean + other.mean,
            sigma: quadrature(self.sigma, other.sigma).sqrt(),
        }
    }
}

impl Add<f64> for Measurement {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            mean: self.mean + other,
            sigma: self.sigma,
        }
    }
}

impl Sub for Measurement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            mean: self.mean - other.mean,
            sigma: quadrature(self.sigma, other.sigma).sqrt(),
        }
    }
}

impl Sub<f64> for Measurement {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            mean: self.mean - other,
            sigma: self.sigma,
        }
    }
}

impl Mul for Measurement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let new_mean = self.mean * other.mean;
        Self {
            mean: new_mean,
            sigma: quadrature(self.sigma / self.mean, other.sigma / other.mean).sqrt() * new_mean,
        }
    }
}

impl Mul<f64> for Measurement {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            mean: self.mean * other,
            sigma: self.sigma * other,
        }
    }
}

impl Div for Measurement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let new_mean = self.mean / other.mean;
        Self {
            mean: new_mean,
            sigma: quadrature(self.sigma / self.mean, other.sigma / other.mean).sqrt() * new_mean,
        }
    }
}

impl Div<f64> for Measurement {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            mean: self.mean / other,
            sigma: self.sigma / other,
        }
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.mean, self.sigma)
    }
}

impl PartialEq for Measurement {
    fn eq(&self, other: &Self) -> bool {
        self.mean == other.mean && self.sigma == other.sigma
    }
}

impl ApproxEq for Measurement {
    type Margin = F64Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.mean.approx_eq(other.mean, margin) && self.sigma.approx_eq(other.sigma, margin)
    }
}

impl From<f64> for Measurement {
    fn from(x: f64) -> Self {
        Measurement {
            mean: x,
            sigma: 0.0,
        }
    }
}

pub fn pow(x: Measurement, y: Measurement) -> Measurement {
    //x^y
    let [xm, ym, sx, sy] = [x.mean, y.mean, x.sigma, y.sigma];
    let dfdx = ym * xm.powf(ym - 1.0);
    let dfdy = xm.ln() * xm.powf(ym);

    Measurement {
        mean: xm.powf(ym),
        sigma: quadrature(dfdx * sx, dfdy * sy).sqrt(),
    }
}

fn quadrature(x: f64, y: f64) -> f64 {
    x * x + y * y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_operations() {
        let x = Measurement::new(1.0, 0.01);
        let y = Measurement::new(2.0, 0.01);

        let added = Measurement::new(3.0, 0.0002f64.sqrt());
        let subtracted = Measurement::new(-1.0, 0.0002f64.sqrt());
        let multiplied = Measurement::new(2.0, 2.0 * (quadrature(0.01 / 1.0, 0.01 / 2.0)).sqrt());
        let divided = Measurement::new(0.5, 0.5 * (quadrature(0.01 / 1.0, 0.01 / 2.0)).sqrt());

        assert!(added.approx_eq(x + y, F64Margin::default()));
        assert!(subtracted.approx_eq(x - y, F64Margin::default()));
        assert!(multiplied.approx_eq(x * y, F64Margin::default()));
        assert!(divided.approx_eq(x / y, F64Margin::default()));
    }
    #[test]
    fn approximate_equality() {
        /* Tests the approximate equality due to floating point errors */
        let x = Measurement::new(1.0, 0.01);
        let y = Measurement::new(1.0, 0.001);
        let x_prime = Measurement::new(1.0, 2.0 * 0.005);

        //A number is equal to itself. Therefore it's also approximately equal to itself
        assert!(x.approx_eq(x, F64Margin::default()));
        //x and y should NOT be approximately equal
        assert_eq!(false, x.approx_eq(y, F64Margin::default()));
        //x and x_prime should be equal
        assert!(x.approx_eq(x_prime, F64Margin::default()));
    }
    #[test]
    fn simple_pow() {
        let x = Measurement::from(2.0);
        let y = Measurement::from(3.0);
        let r = Measurement {
            mean: 8.0,
            sigma: 0.0,
        };
        assert_eq!(pow(x, y), r);
    }

    #[test]
    fn negative_pow() {
        let x = Measurement::from(2.0);
        let y = Measurement::from(-3.0);
        let r = Measurement {
            mean: 1.0 / 8.0,
            sigma: 0.0,
        };
        assert_eq!(pow(x, y), r);
    }
}
