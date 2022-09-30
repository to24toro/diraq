// use crate::algebra::complex::Complex;
use ndarray::prelude::Array2;
use num::complex::Complex;

#[derive(Debug, PartialEq)]
pub struct Gate {
    pub size: usize,
    pub matrix: Array2<Complex<f64>>,
}

impl Gate {
    pub fn new(size: usize) -> Gate {
        Gate {
            size,
            matrix: Array2::<Complex<f64>>::zeros((size, size)),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn matrix(&self) -> &Array2<Complex<f64>> {
        &self.matrix
    }
}
