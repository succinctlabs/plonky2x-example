// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

import "../src/Counter.sol";

contract SimpleCircuitTest is Test {
    SimpleCircuit public simple;
    event CallbackReceived(uint256 requestId, uint32 a_plus_b);

    function setUp() public {
        simple = new SimpleCircuit();
    }

    function testCall() public {
        vm.prank(address(0x852a94F8309D445D27222eDb1E92A4E83DdDd2a8)); // Address of Function Gateway
        bytes memory output = hex"00000003";
        bytes memory context = abi.encode(uint256(1));
        vm.expectEmit();
        emit CallbackReceived(1, uint32(3));
        simple.handleCallback(output, context);
    }
}
