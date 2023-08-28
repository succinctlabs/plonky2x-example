use ethers::providers::{Http, Provider};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::Circuit;
use plonky2x::backend::function::CircuitFunction;
use std::env;

use plonky2x::frontend::eth::vars::AddressVariable;
use plonky2x::frontend::vars::{Bytes32Variable, U32Variable};
use plonky2x::prelude::CircuitBuilder;
use plonky2x::prelude::Variable;

pub struct Function {}

impl CircuitFunction for Function {
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

pub struct StorageProofCircuit {}

impl CircuitFunction for StorageProofCircuit {
    fn build<F, C, const D: usize>() -> Circuit<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        dotenv::dotenv().ok();
        let rpc_url = env::var("RPC_1").unwrap();
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        let mut builder = CircuitBuilder::<F, D>::new();
        builder.set_execution_client(provider);

        let block_hash = builder.evm_read::<Bytes32Variable>();
        let address = builder.evm_read::<AddressVariable>();
        let location = builder.evm_read::<Bytes32Variable>();
        let storage_value = builder.eth_get_storage_at(address, location, block_hash);
        builder.evm_write(storage_value);

        builder.build::<C>()
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");
    Function::cli();
}

#[cfg(test)]
mod tests {
    use ethers::types::{Address, H256};
    use plonky2x::prelude::{GoldilocksField, PoseidonGoldilocksConfig};

    use super::*;

    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    #[test]
    fn test_circuit() {
        let circuit = Function::build::<F, C, D>();
        let mut input = circuit.input();
        input.evm_write::<U32Variable>(0x12345678);
        input.evm_write::<U32Variable>(0x01234567);
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let sum = output.evm_read::<U32Variable>();
        assert_eq!(sum, 0x12345678 + 0x01234567);
    }

    #[test]
    fn test_storage_circuit() {
        let circuit = StorageProofCircuit::build::<F, C, D>();
        let mut input = circuit.input();

        input.evm_write::<Bytes32Variable>(
            "0x281dc31bb78779a1ede7bf0f4d2bc5f07ddebc9f9d1155e413d8804384604bbe"
                .parse::<H256>()
                .unwrap(),
        );
        input.evm_write::<AddressVariable>(
            "0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5"
                .parse::<Address>()
                .unwrap(),
        );
        input.evm_write::<Bytes32Variable>(
            "0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"
                .parse::<H256>()
                .unwrap(),
        );

        let (proof, output) = circuit.prove(&input);

        circuit.verify(&proof, &input, &output);
        let storage_value = output.evm_read::<Bytes32Variable>();
        assert_eq!(
            storage_value,
            "0x0000000000000000000000dd4bc51496dc93a0c47008e820e0d80745476f2201"
                .parse::<H256>()
                .unwrap()
        );
    }
}
