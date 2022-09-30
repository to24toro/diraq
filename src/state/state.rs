use crate::gate::gate::Gate;
// use crate::algebra::complex::Complex;
use crate::algebra::matrix::{index_vec, mask_vec};
use ndarray::{array, prelude::Array1};
use num::complex::Complex;
use std::fmt;

#[derive(Debug)]
pub struct State {
    size: usize,
    pub elements: Array1<Complex<f64>>,
}

impl State {
    pub fn new(size: usize) -> State {
        let mut initial_elements = Array1::from_vec(vec![Complex::new(0., 0.); 1 << size]);
        initial_elements[0] = Complex::new(1., 0.);
        State {
            size,
            elements: initial_elements,
        }
    }

    pub fn apply(&mut self, qubits: &[&usize], gate: &Gate) {
        let dim = qubits.len();

        let masks = mask_vec(qubits);
        for i in 0..(self.elements.len() >> dim) {
            let indices = index_vec(i, qubits, &masks, dim);
            let old_values = indices
                .iter()
                .map(|&i| self.elements[i])
                .collect::<Vec<_>>();
            let new_values = gate.matrix().dot(&Array1::from_vec(old_values));
            for (&i, nv) in indices.iter().zip(new_values.to_vec()) {
                self.elements[i] = nv;
            }
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "qubits: {}, state: {}", self.size, self.elements)
    }
}

#[test]
fn it_should_return_correct_elements_after_apply() {
    let size = 2;

    let mut state = State::new(size);

    let gate = Gate::new(size);

    state.apply(&[&0], &gate);

    let mut correct_elements = Array1::from_vec(vec![Complex::new(0., 0.); 1 << size]);
    correct_elements[0] = Complex::new(1., 0.);

    assert_eq!(correct_elements, state.elements);
}
