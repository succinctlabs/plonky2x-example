use std::env;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::Circuit;
use plonky2x::backend::function::CircuitFunction;

use plonky2x::frontend::vars::U32Variable;
use plonky2x::prelude::CircuitBuilder;
use plonky2x::prelude::Variable;

pub struct SimpleCircuit {}

impl CircuitFunction for SimpleCircuit {
    fn build<F, C, const D: usize>() -> Circuit<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let mut builder = CircuitBuilder::<F, D>::new();

        let a = builder.evm_read::<U32Variable>();
        let b = builder.evm_read::<U32Variable>();
        let c = builder.api.add(a.0 .0, b.0 .0);

        builder.evm_write(U32Variable(Variable(c)));
        builder.build::<C>()
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");
    SimpleCircuit::cli();
}

#[cfg(test)]
mod tests {
    use plonky2x::prelude::{GoldilocksField, PoseidonGoldilocksConfig};

    use super::*;

    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    #[test]
    fn test_circuit() {
        let circuit = SimpleCircuit::build::<F, C, D>();
        let mut input = circuit.input();
        input.evm_write::<U32Variable>(0x12345678);
        input.evm_write::<U32Variable>(0x01234567);
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let sum = output.evm_read::<U32Variable>();
        assert_eq!(sum, 0x12345678 + 0x01234567);
    }
}
