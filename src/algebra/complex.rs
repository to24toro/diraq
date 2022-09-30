//  ref: https://github.com/beneills/quantum/blob/master/src/complex.rs
use num_traits::identities::Zero;
use std::f64::consts::PI;
use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;

#[derive(Clone, Copy, PartialEq)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Complex {
        Complex { real, imaginary }
    }
    pub fn norm(&self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }
    pub fn real(&self) -> f64 {
        self.real
    }
    pub fn imaginary(&self) -> f64 {
        self.imaginary
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imaginary + other.imaginary)
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imaginary * other.imaginary,
            self.real * other.imaginary + self.imaginary * other.real,
        )
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Complex) {
        *self = *self + other;
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Complex) {
        *self = *self * other;
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:+.3} + {:+.3}i", self.real, self.imaginary)
    }
}

impl Zero for Complex {
    fn zero() -> Complex {
        Complex {
            real: 0.,
            imaginary: 0.,
        }
    }

    fn is_zero(&self) -> bool {
        self.real == 0. && self.imaginary == 0.
    }
}
