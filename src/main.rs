mod algebra;
mod circuit;
mod gate;
mod state;
mod validate;

use circuit::circuit::QuantumCircuit;
use gate::base_gates::{DoubleGate, SingleGate};

fn main() {
    let mut qc = QuantumCircuit::new(1);
    qc.H(0);
    println!("{}", qc.state);
    println!("{}", qc.measure(0, 100000));
}
