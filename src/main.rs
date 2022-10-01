mod algebra;
mod circuit;
mod gate;
mod state;
mod validate;

use circuit::circuit::QuantumCircuit;
use gate::base_gates::{DoubleGate, SingleGate};

fn main() {
    println!("hi");
    let mut qc = QuantumCircuit::new(3);
    qc.H(0);
    // qc.apply(&[&0], &SingleGate::H());
    qc.apply(&[&0, &1], &DoubleGate::CNOT());
    qc.apply(&[&0, &2], &DoubleGate::CNOT());
    println!("{}", qc.state);
}
