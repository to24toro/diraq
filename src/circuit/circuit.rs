use crate::gate::base_gates::{DoubleGate, SingleGate, TripleGate};
use crate::gate::gate::Gate;
use crate::state::state::State;
use crate::validate::validate::{
    ctrl_qubit_should_be_different_from_target_qubit, qubit_should_be_less_than_circuit_size,
};

pub struct QuantumCircuit {
    pub state: State,
    size: usize,
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
}
