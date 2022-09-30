mod algebra;
mod circuit;
mod gate;
mod state;

use circuit::circuit::QuantumCircuit;
use gate::base_gates::{DoubleGate, SingleGate};

fn main() {
    println!("hi");
    let mut qc = QuantumCircuit::new(3);
    qc.apply(&[&0], &SingleGate::H());
    qc.apply(&[&0, &1], &DoubleGate::CX());
    qc.apply(&[&0, &2], &DoubleGate::CX());
}
