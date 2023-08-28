// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/CrossChainStorage.sol";
import "../test/Mock.sol";

contract CrossChainStorageScript is Script {
    function run() public {
        // Deploy all contracts
        MockL1Contract l1block = new MockL1Contract();
        uint64 l1_number = uint64(17880427);
        bytes32 l1_block_hash = bytes32(
            0x281dc31bb78779a1ede7bf0f4d2bc5f07ddebc9f9d1155e413d8804384604bbe
        );
        l1block.setBlock(l1_number, l1_block_hash);
        CrossChainStorageSlot c = new CrossChainStorageSlot(address(l1block));

        // This is if you want to use an already deployed contract.
        // CrossChainStorageSlot c = CrossChainStorageSlot(
        //     address(0x2F3E76EFD9Dd0a99B205b477d4f2e440d574cdc9)
        // );

        address addr = 0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5;
        bytes32 location = 0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5;
        c.requestMirrorSlot{value: 30 gwei * 1_000_000}(addr, location);
    }
}
