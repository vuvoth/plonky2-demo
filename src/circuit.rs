use plonky2::{
    field::goldilocks_field::GoldilocksField,
    iop::{
        target::Target,
        witness::{PartialWitness, Witness},
    },
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::{CircuitConfig, CircuitData, VerifierCircuitData},
        config::PoseidonGoldilocksConfig,
    },
};

pub type F = GoldilocksField;
pub type C = PoseidonGoldilocksConfig;

use anyhow::{Ok, Result};

pub struct Circuit {
    pub a: F,
    pub b: F,
    pub output: F,
}

/**
 * prove a^2 * b^2 = c
 * |(0)    (1)      (2)          |
 * | a      a      a * a         |
 * | b      b      b * b         |
 * | a * a  b * b  a * a + b * b |
 */
impl Circuit {
    pub fn make_circuit(&self) -> Result<(Target, Target, Target, CircuitData<F, C, 2>)> {
        let config = CircuitConfig::standard_recursion_zk_config();
        let mut builder = CircuitBuilder::new(config);

        let a = builder.add_virtual_target();
        let b = builder.add_virtual_target();


        // config circuit

        // NOTE: when use mul and add they auto config copy constraint between target.
        let a_square = builder.mul(a, a);
        let b_square = builder.mul(b, b);
        let output = builder.add(a_square, b_square);

        builder.register_public_input(output);
        
        // build circuit
        let circuit = builder.build();

        Ok((a, b, output, circuit))
    }

    pub fn create_prove(
        &self,
        a: Target,
        b: Target,
        output: Target,
        circuit: CircuitData<F, C, 2>,
    ) -> Result<()> {
        let mut pw = PartialWitness::new();
        pw.set_target(output, self.output);
        pw.set_target(a, self.a);
        pw.set_target(b, self.b);
        let proof = circuit.prove(pw).unwrap();
        circuit.verify(proof)
    }
}
