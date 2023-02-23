use plonky2::{
    field::{goldilocks_field::GoldilocksField, field_types::Field},
    iop::{
        target::Target,
        witness::{PartialWitness, Witness},
    },
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::{CircuitConfig, CircuitData},
        config::PoseidonGoldilocksConfig,
    },
};

pub type F = GoldilocksField;
pub type C = PoseidonGoldilocksConfig;

use anyhow::{Ok, Result};

pub struct Circuit {
    pub x: F,
    pub output: F,
}


impl Circuit {
    pub fn make_circuit(&self) -> Result<(Target, Target, CircuitData<F, C, 2>)> {
        // use standard config 
        let config = CircuitConfig::standard_recursion_config();
        // create builder from config
        let mut builder = CircuitBuilder::new(config);

        
        // formula x ^ 3 + x + 5 = output
        // configure circuit

        // we can assume target is node in circuit         
        let x = builder.add_virtual_target();

        // x_square = x * x;
        let x_square = builder.square(x);
        // target x^3 = x * x * x
        let x_cube = builder.mul(x_square, x);
        // target sum_0 = x^3 + x 
        let sum_0 = builder.add(x_cube, x);
        // target output = sum_0 + 5 = x^3 + x ^ 5  
        let output = builder.add_const(sum_0, F::from_canonical_u64(5));
        
        // we set output is public
        builder.register_public_input(output);

        // build circuit
        let circuit = builder.build();

        // output circuit and target
        Ok((x, output, circuit))
    }

    pub fn create_and_verify_proof(
        &self,
        x: Target,
        output: Target,
        circuit: CircuitData<F, C, 2>,
    ) -> Result<()> {
        // partial witness is witness with input only
        let mut pw = PartialWitness::new();
        // we set input value for target
        pw.set_target(output, self.output);
        pw.set_target(x, self.x);
        
        // plonky2 auto compute full witness base on circuit
        let proof = circuit.prove(pw).unwrap();

        // verify proof 
        circuit.verify(proof)
    }
}
