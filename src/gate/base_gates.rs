use crate::gate::gate::Gate;
use ndarray::{array, prelude::Array2};
use num::{complex::Complex, One, Zero};

pub struct SingleGate {}

impl SingleGate {
    pub fn H() -> Gate {
        let sqrt2inv = Complex::new(2.0f64.sqrt().recip(), 0.);

        Gate {
            size: 1,
            matrix: array![[sqrt2inv, sqrt2inv], [sqrt2inv, -sqrt2inv]],
        }
    }

    pub fn X() -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 1,
            matrix: array![[zero, one], [one, zero]],
        }
    }

    pub fn Y() -> Gate {
        let zero = Complex::zero();
        let img_one = Complex::new(0., 1.);

        Gate {
            size: 1,
            matrix: array![[zero, -img_one], [img_one, zero]],
        }
    }

    pub fn Z() -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 1,
            matrix: array![[one, zero], [zero, -one]],
        }
    }

    pub fn P(theta: f64) -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 1,
            matrix: array![[one, zero], [zero, Complex::new(theta.cos(), -theta.sin())]],
        }
    }

    pub fn I() -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 1,
            matrix: array![[one, zero], [zero, one]],
        }
    }

    pub fn RX(theta: f64) -> Gate {
        let cos = Complex::new((theta / 2.).cos(), 0.);
        let isin = Complex::new(0., (theta / 2.).sin());

        Gate {
            size: 1,
            matrix: array![[cos, -isin], [isin, cos]],
        }
    }

    pub fn RY(theta: f64) -> Gate {
        let cos = Complex::new((theta / 2.).cos(), 0.);
        let sin = Complex::new((theta / 2.).sin(), 0.);

        Gate {
            size: 1,
            matrix: array![[cos, -sin], [sin, cos]],
        }
    }

    pub fn RZ(theta: f64) -> Gate {
        let cos = (theta / 2.).cos();
        let sin = (theta / 2.).sin();

        Gate {
            size: 1,
            matrix: array![
                [Complex::new(cos, -sin), Complex::zero()],
                [Complex::zero(), Complex::new(cos, sin)]
            ],
        }
    }
}

pub struct DoubleGate {}

impl DoubleGate {
    pub fn CNOT() -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 2,
            matrix: array![
                [one, zero, zero, zero],
                [zero, one, zero, zero],
                [zero, zero, zero, one],
                [zero, zero, one, zero]
            ],
        }
    }

    pub fn CZ() -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 2,
            matrix: array![
                [one, zero, zero, zero],
                [zero, one, zero, zero],
                [zero, zero, one, zero],
                [zero, zero, zero, -one]
            ],
        }
    }

    pub fn CP(theta: f64) -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 2,
            matrix: array![
                [one, zero, zero, zero],
                [zero, one, zero, zero],
                [zero, zero, one, zero],
                [zero, zero, zero, Complex::new(theta.cos(), -theta.sin())]
            ],
        }
    }

    pub fn SWAP() -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 2,
            matrix: array![
                [one, zero, zero, zero],
                [zero, zero, one, zero],
                [zero, one, zero, zero],
                [zero, zero, zero, one]
            ],
        }
    }
}

pub struct TripleGate {}

impl TripleGate {
    pub fn Toffoli() -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 3,
            matrix: array![
                [one, zero, zero, zero, zero, zero, zero, zero],
                [zero, one, zero, zero, zero, zero, zero, zero],
                [zero, zero, one, zero, zero, zero, zero, zero],
                [zero, zero, zero, one, zero, zero, zero, zero],
                [zero, zero, zero, zero, one, zero, zero, zero],
                [zero, zero, zero, zero, zero, one, zero, zero],
                [zero, zero, zero, zero, zero, zero, zero, one],
                [zero, zero, zero, zero, zero, zero, one, zero]
            ],
        }
    }

    pub fn CCSWAP() -> Gate {
        let zero = Complex::zero();
        let one = Complex::one();

        Gate {
            size: 3,
            matrix: array![
                [one, zero, zero, zero, zero, zero, zero, zero],
                [zero, one, zero, zero, zero, zero, zero, zero],
                [zero, zero, one, zero, zero, zero, zero, zero],
                [zero, zero, zero, one, zero, zero, zero, zero],
                [zero, zero, zero, zero, one, zero, zero, zero],
                [zero, zero, zero, zero, zero, zero, one, zero],
                [zero, zero, zero, zero, zero, one, zero, zero],
                [zero, zero, zero, zero, zero, zero, zero, one]
            ],
        }
    }
}

// #[test]
//  it should implement partialeq for array2 later.
// https://github.com/rust-ndarray/ndarray/issues/294
// fn it_should_give_hadamard_gate() {
//     let hadamard = SingleGate::H();

//     assert_eq!(hadamard.matrix(),
//         [[0.7071067811865475+0i, 0.7071067811865475+0i],
//         [0.7071067811865475+0i, -0.7071067811865475+-0i]]);
// }
