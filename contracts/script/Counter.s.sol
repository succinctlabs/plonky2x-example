// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/Counter.sol";

contract CounterScript is Script {
    function run() public {
        vm.broadcast();
        SimpleCircuit s = SimpleCircuit(
            address(0xA8963BB3cAfdd188e323cff40f667E875e3C9fC7)
        );
        s.requestAddition{value: 30 gwei * 1_000_000}(2, 3);
    }
}
