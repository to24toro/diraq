//  ref: https://github.com/hajifkd/rusq/blob/master/src/simulator/simulator.rs

pub fn masks(qubit: usize) -> (usize, usize) {
    let upper_mask = 0xFFFF_FFFF_FFFF_FFFFusize << (qubit + 1);
    let lower_mask = !(0xFFFF_FFFF_FFFF_FFFFusize << qubit);
    (upper_mask, lower_mask)
}

pub fn indices(
    index: usize,
    qubit: &usize,
    upper_mask: usize,
    lower_mask: usize,
) -> (usize, usize) {
    let index_zero = ((index << 1) & upper_mask) | (index & lower_mask);
    let index_one = index_zero | (1usize << qubit);
    (index_zero, index_one)
}

pub fn mask_vec(qubits: &[&usize]) -> Vec<usize> {
    let mut qubits = qubits.to_owned();
    qubits.sort();
    let mut masked_qubits_elements = vec![0; qubits.len() + 1];

    masked_qubits_elements[0] = 0xFFFF_FFFF_FFFF_FFFFusize << (qubits[qubits.len() - 1] + 1);

    for i in 1..qubits.len() {
        masked_qubits_elements[i] = (0xFFFF_FFFF_FFFF_FFFFusize
            << (qubits[qubits.len() - i - 1] + 1))
            | (!(0xFFFF_FFFF_FFFF_FFFFusize << (qubits[qubits.len() - i])));
    }

    masked_qubits_elements[qubits.len()] = !(0xFFFF_FFFF_FFFF_FFFFusize << qubits[0]);

    masked_qubits_elements
}

pub fn index_vec(index: usize, qubits: &[&usize], mask: &[usize], dim: usize) -> Vec<usize> {
    let imask = (0..dim + 1)
        .map(|s| (index << (dim - s)) & mask[s])
        .fold(0, |acc, m| acc | m);
    (0..1 << dim)
        .map(|i| {
            (0..dim).fold(imask, |acc, j| {
                acc | ((i >> (dim - 1 - j) & 0b1) << qubits[j])
            })
        })
        .collect()
}
