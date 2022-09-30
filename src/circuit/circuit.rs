use crate::gate::gate::Gate;
use crate::state::state::State;

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
}
