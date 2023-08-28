// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

import {CrossChainStorageSlot} from "../src/CrossChainStorage.sol";
import {MockL1Contract} from "./Mock.sol";

contract CrossChainStorageSlotTest is Test {
    MockL1Contract public l1block;
    CrossChainStorageSlot public mirror;
    event StorageSlotMirrored(
        address addr,
        bytes32 location,
        bytes32 value,
        uint256 blockNumber
    );

    function setUp() public {
        l1block = new MockL1Contract();
        mirror = new CrossChainStorageSlot(address(l1block));
    }

    function test_CrossChainStorage() public {
        // These values are taken from Ethereum block https://etherscan.io/block/17880427
        uint64 l1_number = uint64(17880427);
        bytes32 l1_block_hash = bytes32(
            0x281dc31bb78779a1ede7bf0f4d2bc5f07ddebc9f9d1155e413d8804384604bbe
        );
        l1block.setBlock(l1_number, l1_block_hash);
        address addr = 0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5;
        bytes32 location = 0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5;
        bytes32 storage_value = 0x0000000000000000000000dd4bc51496dc93a0c47008e820e0d80745476f2201;
        // TODO: have to use vm.etch with the MockGateway here to test
        // mirror.requestMirrorSlot{value: 30 gwei * 1_000_000}(addr, location);

        vm.expectEmit();
        emit StorageSlotMirrored(addr, location, storage_value, l1_number);

        vm.prank(address(0x852a94F8309D445D27222eDb1E92A4E83DdDd2a8)); // Address of Function Gateway
        bytes memory context = abi.encode(l1_number, addr, location);
        mirror.storeSlotValue(
            hex"0000000000000000000000dd4bc51496dc93a0c47008e820e0d80745476f2201",
            context
        );
    }
}
