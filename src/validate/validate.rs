pub fn qubit_should_be_less_than_circuit_size(qubit: &usize, circuit_size: &usize) {
    assert_eq!(true, qubit < circuit_size);
}

pub fn ctrl_qubit_should_be_different_from_target_qubit(ctrl_qubit: &usize, target_qubit: &usize) {
    assert_eq!(true, ctrl_qubit != target_qubit);
}
