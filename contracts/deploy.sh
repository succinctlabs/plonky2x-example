source ../.env
forge create src/Counter.sol:SimpleCircuit --rpc-url $RPC_1 --private-key $PRIVATE_KEY --verify --etherscan-api-key $ETHERSCAN_API_KEY
