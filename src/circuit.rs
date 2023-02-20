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
        let config = CircuitConfig::standard_recursion_zk_config();
        let mut builder = CircuitBuilder::new(config);

        // x ^ 3 + x + 5 = 35
        // config circuit

        let x = builder.add_virtual_target();

        let x_cube = builder.cube(x);
        let sum_0 = builder.add(x_cube, x);
        let output = builder.add_const(sum_0, F::from_canonical_u32(5));
        

        builder.register_public_input(output);

        // build circuit
        let circuit = builder.build();

        Ok((x, output, circuit))
    }

    pub fn create_and_verify_proof(
        &self,
        x: Target,
        output: Target,
        circuit: CircuitData<F, C, 2>,
    ) -> Result<()> {
        let mut pw = PartialWitness::new();
        pw.set_target(output, self.output);
        pw.set_target(x, self.x);
        
        let proof = circuit.prove(pw).unwrap();

        circuit.verify(proof)
    }
}
