use circuit::{F, Circuit};
use plonky2::field::field_types::Field;

mod circuit;

fn main() {
    let circuit = Circuit {
        x: F::from_canonical_u64(3),
        output: F::from_canonical_u64(35)
    };

    let (x,  output, circuit_instance) = circuit.make_circuit().unwrap();


    circuit.create_and_verify_proof(x, output, circuit_instance).unwrap()
}