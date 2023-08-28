# plonky2x-example
An example of how to build an end-to-end dApp with plonky2x


### Step by step

1. Make sure you have Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
2. Make sure you have Foundry installed for smart contract development (`curl -L https://foundry.paradigm.xyz | bash`)
3. Make sure environment variables in the top-level `.env` are set by following the variables in `.env.example`
4. Make sure the circuits build and the tests pass by running `cargo test`
5. Make sure contracts build and the tests pass by running `forge test` in the `contracts` folder
6. Go to `alpha.succinct.xyz` and login and click `new` to connect this repo to Succinct
7. Make a new "release" of your circuit and deploy the verifier on-chain once the release is done
8. Take the `function_id` of your deployed contract and change it in your smart contracts (`src/Counter.sol`)
9. Deploy your smart contract