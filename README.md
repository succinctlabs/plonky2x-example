# plonky2x-example

An example of how to build an end-to-end dApp with plonky2x

### Setup

1. Make sure you have Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
2. Use nightly Rust (`rustup override set nightly`)
3. Make sure you have Foundry installed for smart contract development (`curl -L https://foundry.paradigm.xyz | bash`)
4. Copy `.env.example` to a new file called `.env` and fill in the proper values

### Writing your circuit

1. Edit `circuit/main.rs` to modify your circuit logic as you wish (or leave the example as is)
2. As you develop your circuit, make sure the circuits build and the tests pass by running `cargo test`. Add tests as needed to validate your circuit logic.

### Writing your smart contract

Your smart contract is the entrypoint for users to request proofs and then process the callback when it comes back onchain. The proof verifier is handled separately through the platform + FunctionGateway.

1. Edit `contracts/src/Addition.sol` to modify your contract logic as you wish, and `contracts/test/Addition.t.sol` correspondingly
2. As you develop, make sure contracts build and the tests pass by running `forge test` in the `contracts` folder

### Deploying your circuit

Once your circuit is ready, register your project and create a release on Succinct X. The platform will then build your circuit and have it ready for generating proofs.

1. Push your code to github.
2. Go to [`alpha.succinct.xyz`](https://alpha.succinct.xyz) and login and click `New` to connect this repo to Succinct
3. Go to releases and click `New` to make a new "release" of your circuit

After creating a release, you can deploy your circuit's verifier to whatever chains you'd like. For now we'll just do Goerli.

3. Make sure you have MetaMask or another browser wallet setup and a wallet with some Goerli ETH.
4. On the platform, go to your project's deployments and then click `New`. Enter anything in `Function Name`, select `Goerli`, and Deploy.

After a few minutes, the deployment should be finished. Go back to Deployments and your new deployment should be there.

5. Take the `function_id` of your deployed contract and put it in your smart contracts (`contracts/src/Addition.sol`)

### Deploying your smart contract

1. Deploy your smart contract by using `./deployAddition.sh` in `contracts/`
2. Edit `contracts/script/Addition.s.sol` with your contract's deployed address
3. Request a proof by using `./requestAddition.sh` in `contracts/` to trigger a request on-chain
