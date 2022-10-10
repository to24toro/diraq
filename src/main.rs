mod algebra;
mod circuit;
mod gate;
mod state;
mod validate;

use circuit::circuit::QuantumCircuit;
use gate::base_gates::{DoubleGate, SingleGate};

fn main() {
    let mut qc = QuantumCircuit::new(3);
    qc.X(0);
    qc.X(1);
    // qc.X(2);
    qc.CCSWAP(0, 1, 2);

    println!("{}", qc.state);
}
