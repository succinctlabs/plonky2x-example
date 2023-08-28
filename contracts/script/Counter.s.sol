// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/Counter.sol";

contract CounterScript is Script {
    function run() public {
        vm.broadcast();
        SimpleCircuit s = SimpleCircuit(
            address(0x2F3E76EFD9Dd0a99B205b477d4f2e440d574cdc9)
        );
        s.requestAddition{value: 30 gwei * 1_000_000}(2, 3);
    }
}
