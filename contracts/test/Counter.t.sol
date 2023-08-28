// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
// import "forge-std/Vm.sol";
import "../src/Counter.sol";

contract MockL1Contract {
    uint256 public number;
    bytes32 public hash_;

    function setBlock(
        bytes32 hash_,
        uint256 number
    ) external view returns (bytes32) {
        hash_ = hash_;
        number = number;
    }
}

contract CrossChainStorageSlotTest is Test {
    CrossChainStorageSlot public counter;
    event CallbackReceived(uint256 requestId, uint32 a_plus_b);

    function setUp() public {
        counter = new CrossChainStorageSlot();
    }

    function testCall() public {
        vm.prank(address(0x852a94F8309D445D27222eDb1E92A4E83DdDd2a8)); // Address of Function Gateway
        bytes memory output = hex"00000003";
        bytes memory context = abi.encode(uint256(1));
        vm.expectEmit();
        emit CallbackReceived(1, uint32(3));
        counter.handleCallback(output, context);
        // assertEq(counter.number(), 1);
    }
}
