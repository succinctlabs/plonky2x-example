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
            "971a07f522aa78292e76c6f7868e95212b75e68c94ca077e6207722795d80a61"
        ));
        let siblings = [
            builder.constant::<Bytes32Variable>(bytes32!(
                "5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2"
            )),
            builder.constant::<Bytes32Variable>(bytes32!(
                "2b07d07815e57c23883128aa268a683b3b39aca921fa5f247e9a30c4035d7107"
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
