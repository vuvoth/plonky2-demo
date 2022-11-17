use circuit::{F, Circuit};
use plonky2::field::field_types::Field;

mod circuit;

fn main() {
    let circuit = Circuit {
        a: F::from_canonical_u64(2),
        b: F::from_canonical_u64(2),        
        output: F::from_canonical_u64(2 * 2 + 2 * 2)
    };

    let (a, b, output, circuit_instance) = circuit.make_circuit().unwrap();

    circuit.create_prove(a, b, output, circuit_instance).unwrap()
}