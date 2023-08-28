pragma solidity ^0.8.13;

contract MockL1Contract {
    uint64 public number;
    bytes32 public hash;

    function setBlock(uint64 _number, bytes32 _hash) external {
        hash = _hash;
        number = _number;
    }
}
