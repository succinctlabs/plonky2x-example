use succinctlabs::circuit::CircuitBuilderX;
use succinctlabs::circuit::CircuitTrait;

pub struct MyCircuit {
}

impl CircuitTrait for MyCircuit {
    pub fn define(builder: CircuitBuilderX) {
        let l1_block_hash = builder.evm_read::<Bytes32Variable>();
        let address = builder.evm_read::<Address>();
        let location = builder.evm_read::<Bytes32Variable>();
        let value = builder.eth_getStorageAt(l1_block_hash, address, location);
        builder.evm_write(value);
    }
}

mod test {
    use super::*;
    use succinctlabs::circuit::CircuitBuilderX;
    use ethers::core::types::{Address, H256};

    #[test]
    fn test_get_proof() {
        // TODO test the getting of a simple storage proof
    }

    #[test]
    fn test_circuit() {
        let mut builder = CircuitBuilderX::new();
        MyCircuit::define(&mut builder);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // To test your circuit, write to the inputs
        // Note this needs to be done after building the circuit.
        let mut input = circuit.input();
        input.write::<Byte32Variable>();
        input.write::<AddressVariable>();
        input.write::<Bytes32Variable>();

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let sum = output.read::<Bytes32Variable>();
        println!("{}", sum.0);
    }
}

