use crate::algebra::matrix::{indices, masks};
use crate::circuit;
use crate::gate::base_gates::{DoubleGate, SingleGate, TripleGate};
use crate::gate::gate::Gate;
use crate::state::state::State;
use crate::validate::validate::{
    ctrl_qubit_should_be_different_from_target_qubit, qubit_should_be_less_than_circuit_size,
};
use num::complex::ComplexFloat;
use num::{complex::Complex, One, Zero};
use rand;
use std::f64::consts::PI;
use std::f64::EPSILON;
use std::fmt;

pub struct QuantumCircuit {
    pub state: State,
    size: usize,
}

pub struct MeasurementResult {
    zero: usize,
    one: usize,
    shots: usize,
    qubit: usize,
}

enum Basic {
    Zero,
    One,
}

impl MeasurementResult {
    fn new(qubit: usize, shots: usize) -> MeasurementResult {
        MeasurementResult {
            zero: 0,
            one: 0,
            shots,
            qubit,
        }
    }
}

impl fmt::Display for MeasurementResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "measured qubit: {}, '0': {}, '1': {}, 'shots': {}",
            self.qubit, self.zero, self.one, self.shots
        )
    }
}

impl QuantumCircuit {
    pub fn new(size: usize) -> QuantumCircuit {
        QuantumCircuit {
            state: State::new(size),
            size,
        }
    }

    pub fn apply(&mut self, qubits: &[&usize], gate: &Gate) {
        // assert_eq!(self.size, gate.size());
        for qubit in qubits.iter() {
            assert_eq!(true, qubit <= &&self.size);
        }
        self.state.apply(qubits, gate);
    }

    pub fn measure(&mut self, qubit: usize, shots: usize) -> MeasurementResult {
        let mut result = MeasurementResult::new(qubit, shots);
        let qubit_ref = qubit.to_owned();
        let (upper_mask, lower_mask) = masks(qubit_ref);
        let zero_norm: f64 = (0..(self.state.elements.len() >> 1))
            .map(|i| self.state.elements[indices(i, &qubit, upper_mask, lower_mask).0].norm_sqr())
            .sum();
        for _ in 0..shots {
            if zero_norm > rand::random::<f64>() {
                let zero_prob = zero_norm.sqrt();
                for i in 0..(self.state.elements.len() >> 1) {
                    let (ith_zero, ith_one) = indices(i, &qubit, upper_mask, lower_mask);
                    self.state.elements[ith_zero] /= zero_prob;
                    self.state.elements[ith_one] = Complex::new(0., 0.);
                }
                result.zero += 1;
            } else {
                let one_prob = (1. - zero_norm).sqrt();
                for i in 0..(self.state.elements.len() >> 1) {
                    let (ith_zero, ith_one) = indices(i, &qubit, upper_mask, lower_mask);
                    self.state.elements[ith_zero] /= one_prob;
                    self.state.elements[ith_one] = Complex::new(0., 0.);
                }
                result.one += 1;
            }
        }
        result
    }

    pub fn H(&mut self, qubit: usize) {
        qubit_should_be_less_than_circuit_size(&qubit, &self.size);
        self.apply(&[&qubit], &SingleGate::H());
    }

    pub fn X(&mut self, qubit: usize) {
        qubit_should_be_less_than_circuit_size(&qubit, &self.size);
        self.apply(&[&qubit], &SingleGate::X());
    }

    pub fn Y(&mut self, qubit: usize) {
        qubit_should_be_less_than_circuit_size(&qubit, &self.size);
        self.apply(&[&qubit], &SingleGate::Y());
    }

    pub fn Z(&mut self, qubit: usize) {
        qubit_should_be_less_than_circuit_size(&qubit, &self.size);
        self.apply(&[&qubit], &SingleGate::Z());
    }

    pub fn P(&mut self, qubit: usize, theta: f64) {
        qubit_should_be_less_than_circuit_size(&qubit, &self.size);
        self.apply(&[&qubit], &SingleGate::P(theta));
    }

    pub fn I(&mut self, qubit: usize) {
        qubit_should_be_less_than_circuit_size(&qubit, &self.size);
        self.apply(&[&qubit], &SingleGate::I());
    }

    pub fn CNOT(&mut self, ctrl_qubit: usize, target_qubit: usize) {
        qubit_should_be_less_than_circuit_size(&ctrl_qubit, &self.size);
        qubit_should_be_less_than_circuit_size(&target_qubit, &self.size);
        ctrl_qubit_should_be_different_from_target_qubit(&ctrl_qubit, &target_qubit);
        self.apply(&[&ctrl_qubit, &target_qubit], &DoubleGate::CNOT());
    }

    pub fn CZ(&mut self, ctrl_qubit: usize, target_qubit: usize) {
        qubit_should_be_less_than_circuit_size(&ctrl_qubit, &self.size);
        qubit_should_be_less_than_circuit_size(&target_qubit, &self.size);
        ctrl_qubit_should_be_different_from_target_qubit(&ctrl_qubit, &target_qubit);
        self.apply(&[&ctrl_qubit, &target_qubit], &DoubleGate::CZ());
    }

    pub fn SWAP(&mut self, ctrl_qubit: usize, target_qubit: usize) {
        qubit_should_be_less_than_circuit_size(&ctrl_qubit, &self.size);
        qubit_should_be_less_than_circuit_size(&target_qubit, &self.size);
        ctrl_qubit_should_be_different_from_target_qubit(&ctrl_qubit, &target_qubit);
        self.apply(&[&ctrl_qubit, &target_qubit], &DoubleGate::SWAP());
    }

    pub fn CP(&mut self, ctrl_qubit: usize, target_qubit: usize, theta: f64) {
        qubit_should_be_less_than_circuit_size(&ctrl_qubit, &self.size);
        qubit_should_be_less_than_circuit_size(&target_qubit, &self.size);
        ctrl_qubit_should_be_different_from_target_qubit(&ctrl_qubit, &target_qubit);
        self.apply(&[&ctrl_qubit, &target_qubit], &DoubleGate::CP(theta));
    }

    pub fn Toffoli(&mut self, ctrl_qubit1: usize, ctrl_qubit2: usize, target_qubit: usize) {
        qubit_should_be_less_than_circuit_size(&ctrl_qubit1, &self.size);
        qubit_should_be_less_than_circuit_size(&ctrl_qubit2, &self.size);
        qubit_should_be_less_than_circuit_size(&target_qubit, &self.size);
        ctrl_qubit_should_be_different_from_target_qubit(&ctrl_qubit1, &target_qubit);
        ctrl_qubit_should_be_different_from_target_qubit(&ctrl_qubit2, &target_qubit);
        self.apply(
            &[&ctrl_qubit1, &ctrl_qubit2, &target_qubit],
            &TripleGate::Toffoli(),
        );
    }

    pub fn CCSWAP(&mut self, ctrl_qubit1: usize, ctrl_qubit2: usize, target_qubit: usize) {
        qubit_should_be_less_than_circuit_size(&ctrl_qubit1, &self.size);
        qubit_should_be_less_than_circuit_size(&ctrl_qubit2, &self.size);
        qubit_should_be_less_than_circuit_size(&target_qubit, &self.size);
        ctrl_qubit_should_be_different_from_target_qubit(&ctrl_qubit1, &target_qubit);
        ctrl_qubit_should_be_different_from_target_qubit(&ctrl_qubit2, &target_qubit);
        self.apply(
            &[&ctrl_qubit1, &ctrl_qubit2, &target_qubit],
            &TripleGate::CCSWAP(),
        );
    }

    pub fn QFT(&mut self, from_qubit: usize, to_qubit: usize) {
        qubit_should_be_less_than_circuit_size(&from_qubit, &self.size);
        qubit_should_be_less_than_circuit_size(&to_qubit, &self.size);
        let qubit_len = to_qubit - from_qubit + 1;
        for q in 0..qubit_len {
            self.apply(&[&q], &SingleGate::H());
            let mut phase = PI;
            for qq in 0..qubit_len - q - 1 {
                phase /= 2.;
                self.CP(qq + q + 1, q, phase);
            }
        }

        let mut id = 0;
        while id < qubit_len - id - 1 {
            self.SWAP(id, qubit_len - id - 1);
            id += 1;
        }
    }
}

#[test]
fn X_test() {
    let mut qc = QuantumCircuit::new(1);
    qc.X(0);
    assert_eq!(Complex::zero(), qc.state.elements[0]);
    assert_eq!(Complex::one(), qc.state.elements[1]);

    let mut qc = QuantumCircuit::new(1);
    assert_eq!(Complex::one(), qc.state.elements[0]);
    assert_eq!(Complex::zero(), qc.state.elements[1]);
}

#[test]
fn Y_test() {
    let mut qc = QuantumCircuit::new(1);
    qc.Y(0);
    assert_eq!(Complex::zero(), qc.state.elements[0]);
    assert_eq!(Complex::new(0., 1.), qc.state.elements[1]);

    let mut qc = QuantumCircuit::new(1);
    qc.X(0);
    qc.Y(0);
    assert_eq!(Complex::new(0., -1.), qc.state.elements[0]);
    assert_eq!(Complex::zero(), qc.state.elements[1]);
}

#[test]
fn Z_test() {
    let mut qc = QuantumCircuit::new(1);
    qc.Z(0);
    assert_eq!(Complex::one(), qc.state.elements[0]);
    assert_eq!(Complex::zero(), qc.state.elements[1]);

    let mut qc = QuantumCircuit::new(1);
    qc.X(0);
    qc.Z(0);
    assert_eq!(Complex::zero(), qc.state.elements[0]);
    assert_eq!(-Complex::one(), qc.state.elements[1]);
}

#[test]
fn P_test() {
    let mut qc = QuantumCircuit::new(1);
    qc.P(0, PI);
    assert_eq!(Complex::one(), qc.state.elements[0]);
    assert_eq!(Complex::zero(), qc.state.elements[1]);

    let mut qc = QuantumCircuit::new(1);
    qc.X(0);
    qc.P(0, PI);
    assert_eq!(Complex::zero(), qc.state.elements[0]);
    assert_eq!(-1. - qc.state.elements[1].re() < EPSILON, true);

    let mut qc = QuantumCircuit::new(1);
    qc.X(0);
    qc.P(0, PI / 2.);
    assert_eq!(Complex::zero(), qc.state.elements[0]);
    assert_eq!(-1. - qc.state.elements[1].im() < EPSILON, true);
}

#[test]
fn H_test() {
    let mut qc = QuantumCircuit::new(1);
    let sqrt2inv = Complex::new(2.0f64.sqrt().recip(), 0.);
    qc.H(0);
    assert_eq!(sqrt2inv, qc.state.elements[0]);
    assert_eq!(sqrt2inv, qc.state.elements[1]);

    let mut qc = QuantumCircuit::new(1);
    qc.X(0);
    qc.H(0);
    assert_eq!(sqrt2inv, qc.state.elements[0]);
    assert_eq!(-sqrt2inv, qc.state.elements[1]);
}

#[test]
fn CNOT_test() {
    let mut qc = QuantumCircuit::new(2);
    qc.X(0);
    qc.CNOT(0, 1);
    assert_eq!(Complex::zero(), qc.state.elements[1]);
    assert_eq!(Complex::one(), qc.state.elements[3]);

    let mut qc = QuantumCircuit::new(2);
    qc.X(0);
    qc.X(1);
    qc.CNOT(0, 1);
    assert_eq!(Complex::one(), qc.state.elements[1]);
    assert_eq!(Complex::zero(), qc.state.elements[3]);
}

#[test]
fn CZ_test() {
    let mut qc = QuantumCircuit::new(2);
    qc.X(0);
    qc.X(1);
    qc.CZ(0, 1);
    assert_eq!(-Complex::one(), qc.state.elements[3]);
}
#[test]
fn CP_test() {
    let mut qc = QuantumCircuit::new(2);
    qc.X(0);
    qc.X(1);
    qc.CP(0, 1, PI);
    assert_eq!(-1. - qc.state.elements[3].re() < EPSILON, true);

    let mut qc = QuantumCircuit::new(2);
    qc.X(0);
    qc.X(1);
    qc.CP(0, 1, PI / 2.);
    assert_eq!(-1. - qc.state.elements[3].im() < EPSILON, true);
}

#[test]
fn SWAP_test() {
    let mut qc = QuantumCircuit::new(2);
    qc.X(0);
    qc.SWAP(0, 1);
    assert_eq!(Complex::zero(), qc.state.elements[1]);
    assert_eq!(Complex::one(), qc.state.elements[2]);

    let mut qc = QuantumCircuit::new(2);
    qc.X(1);
    qc.SWAP(0, 1);
    assert_eq!(Complex::one(), qc.state.elements[1]);
    assert_eq!(Complex::zero(), qc.state.elements[2]);
}

#[test]
fn Toffoli_test() {
    let mut qc = QuantumCircuit::new(3);
    qc.X(0);
    qc.X(1);
    qc.Toffoli(0, 1, 2);
    assert_eq!(Complex::zero(), qc.state.elements[3]);
    assert_eq!(Complex::one(), qc.state.elements[7]);

    let mut qc = QuantumCircuit::new(3);
    qc.X(0);
    qc.X(1);
    qc.X(2);
    qc.Toffoli(0, 1, 2);
    assert_eq!(Complex::one(), qc.state.elements[3]);
    assert_eq!(Complex::zero(), qc.state.elements[7]);
}

#[test]
fn CCSWAP_test() {
    let mut qc = QuantumCircuit::new(3);
    qc.X(0);
    qc.X(1);
    qc.CCSWAP(0, 1, 2);
    assert_eq!(Complex::zero(), qc.state.elements[4]);
    assert_eq!(Complex::one(), qc.state.elements[5]);

    let mut qc = QuantumCircuit::new(3);
    qc.X(0);
    qc.X(1);
    qc.X(2);
    qc.CCSWAP(0, 1, 2);
    assert_eq!(Complex::one(), qc.state.elements[7]);
}

#[test]
fn QFT_test() {
    let circuit_size: usize = 3;
    let circuit_size_f64 = (1 << circuit_size) as f64;
    let mut qc = QuantumCircuit::new(circuit_size);
    let sqrt_circuit_size_inv = circuit_size_f64.sqrt().recip();
    qc.QFT(0, circuit_size - 1);
    for i in 0..1 << circuit_size {
        assert_eq!(
            sqrt_circuit_size_inv - qc.state.elements[i].re() < EPSILON,
            true
        );
    }
}
