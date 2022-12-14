mod algebra;
mod circuit;
mod gate;
mod state;
mod validate;

use circuit::circuit::QuantumCircuit;
use gate::base_gates::{DoubleGate, SingleGate};

fn main() {
    let mut qc = QuantumCircuit::new(3);
    qc.QFT(0, 2);
    println!("{}", qc.state);

    // qc.H(0);
    // qc.CNOT(0, 1);
    // qc.CNOT(0, 2);
    // println!("{}", qc.state);
    // println!("{}", qc.measure(1, 1));
    // println!("{}", qc.state);
    // println!("{}", qc.measure(0, 1));
}
