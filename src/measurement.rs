use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use float_cmp::{ApproxEq, F64Margin};


#[derive(Debug, Clone, Copy)]
pub struct Measurement {
    mean: f64,
    sigma: f64
}

impl Measurement {
    pub fn new(mean: f64, sigma: f64) -> Measurement {
        Measurement {
            mean,
            sigma
        }
    }
}

impl Add for Measurement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            mean: self.mean + other.mean,
            sigma: quadrature(self.sigma, other.sigma).sqrt()
        }
    }
}

impl Sub for Measurement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            mean: self.mean - other.mean,
            sigma: quadrature(self.sigma, other.sigma).sqrt()
        }
    }
}

impl Mul for Measurement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let new_mean = self.mean * other.mean;
        Self {
            mean: new_mean,
            sigma: quadrature(self.sigma/self.mean, other.sigma/other.mean).sqrt() * new_mean
        }
    }
}

impl Div for Measurement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let new_mean = self.mean / other.mean;
        Self {
            mean: new_mean,
            sigma: quadrature(self.sigma/self.mean, other.sigma/other.mean).sqrt() * new_mean
        }
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.mean, self.sigma)
    }
}

impl ApproxEq for Measurement {
    type Margin = F64Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.mean.approx_eq(other.mean, margin)
        && self.sigma.approx_eq(other.sigma, margin)
    }
}

fn quadrature(x: f64, y:f64) -> f64 {
    x*x + y*y
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
        let multiplied = Measurement::new(2.0, 2.0*(quadrature(0.01/1.0, 0.01/2.0)).sqrt());
        let divided = Measurement::new(0.5, 0.5*(quadrature(0.01/1.0, 0.01/2.0)).sqrt());

        
        assert!(added.approx_eq(x+y, F64Margin::default()));
        assert!(subtracted.approx_eq(x-y, F64Margin::default()));
        assert!(multiplied.approx_eq(x*y, F64Margin::default()));
        assert!(divided.approx_eq(x/y, F64Margin::default()));
    }
    #[test]
    fn approximate_equality() {
        /* Tests the approximate equality due to floating point errors */
        let x = Measurement::new(1.0, 0.01);
        let y = Measurement::new(1.0, 0.001);
        let x_prime = Measurement::new(1.0, 2.0 * 0.005);

        //A number is equal to itself. Therefore it's approximately equal to itself
        assert!(x.approx_eq(x, F64Margin::default()));
        //x and y should NOT be approximately equal
        assert_eq!(false, x.approx_eq(y, F64Margin::default()));
        //x and x_prime should be equal
        assert!(x.approx_eq(x_prime, F64Margin::default()));
    }
}