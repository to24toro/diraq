mod algebra;
mod circuit;
mod gate;
mod state;
mod validate;

use circuit::circuit::QuantumCircuit;
use gate::base_gates::{DoubleGate, SingleGate};

fn exec_qft() {
    let mut qc = QuantumCircuit::new(3);
    qc.QFT(0, 2);
    println!("{}", qc.state);
}