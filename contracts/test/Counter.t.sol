// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Counter.sol";

contract MockL1Contract {
    uint256 public number;
    bytes32 public hash_;

    function setBlock(bytes32 hash_, uint256 number) external view returns (bytes32) {
        hash_ = hash_;
        number = number;
    }
}

contract CrossChainStorageSlot is Test {
    CrossChainStorageSlot public counter;

    function setUp() public {
        counter = new CrossChainStorageSlot();
        counter.setNumber(0);
    }

    function testCall() public {
        counter.increment();
        assertEq(counter.number(), 1);
    }

