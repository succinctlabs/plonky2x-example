use ethers::providers::{Http, Provider};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::Circuit;
use plonky2x::backend::function::CircuitFunction;
use plonky2x::utils::bytes32;
use std::env;

use plonky2x::frontend::eth::vars::AddressVariable;
use plonky2x::frontend::vars::{Bytes32Variable, U32Variable};
use plonky2x::prelude::CircuitBuilder;
use plonky2x::prelude::Variable;

pub struct U32AddFunction {}

impl CircuitFunction for U32AddFunction {
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

pub struct Keccak256MerkleProofFunction {}

impl CircuitFunction for Keccak256MerkleProofFunction {
    fn build<F, C, const D: usize>() -> Circuit<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let mut builder = CircuitBuilder::<F, D>::new();

        // Imagine a binary merkle tree with leaves 0, 1, 2, 3 using the keccak256 hash function
        // on 32 bit words.

        let leaf = builder.constant::<Bytes32Variable>(bytes32!(
            "0000000000000000000000000000000000000000000000000000000000000000"
        ));
        let root = builder.constant::<Bytes32Variable>(bytes32!(
            "2c24f92f65cdd0fde0264c1f41fadf17cb35cdffeaca769e5673e72b072be707"
        ));
        let siblings = [
            builder.constant::<Bytes32Variable>(bytes32!(
                "b10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf6"
            )),
            builder.constant::<Bytes32Variable>(bytes32!(
                "c5fd106a8e5214837c622e5fdef112b1d83ad6de66beafb53451c77843c9d04e"
            )),
        ];

        // TODO: Verify the merkle proof using builder.keccak256 and builder.assert_is_equal.
        // Note that assert_is_equal operates over Variable, not Bytes32Variable so for now you
        // will need to do something like: builder.assert_is_equal(a.variables(), b.variables()) if
        // a and b are of type Bytes32Variable.

        builder.build::<C>()
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");
    U32AddFunction::cli();
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
        let circuit = U32AddFunction::build::<F, C, D>();
        let mut input = circuit.input();
        input.evm_write::<U32Variable>(0x12345678);
        input.evm_write::<U32Variable>(0x01234567);
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let sum = output.evm_read::<U32Variable>();
        assert_eq!(sum, 0x12345678 + 0x01234567);
    }

    #[test]
    fn test_keccak256_merkle_proof() {
        let circuit = Keccak256MerkleProofFunction::build::<F, C, D>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
