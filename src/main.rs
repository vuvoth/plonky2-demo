use circuit::{F, Circuit};
use plonky2::field::field_types::Field;

mod circuit;

fn main() {

    // create circuit struct
    // x = 5 
    // verify x^3 +x + 5 = 35 
    let circuit = Circuit {
        x: F::from_canonical_u64(3),
        output: F::from_canonical_u64(35)
    };

    // make circuit 
    let (x,  output, circuit_instance) = circuit.make_circuit().unwrap();

    // prove and verify data
    circuit.create_and_verify_proof(x, output, circuit_instance).unwrap()
}