// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {IFunctionGateway} from "@succinct-sdk/interfaces/IFunctionGateway.sol";
import {OutputReader} from "@succinct-sdk/libraries/OutputReader.sol";


interface L1Block {
    function hash() external view returns (bytes32);
}

contract CrossChainStorageSlot {
    mapping(bytes32 => bytes32) public storageSlotToValue;
    mapping(bytes32 => uint256) public lastUpdated;

    address public immutable FUNCTION_GATEWAY;
    bytes32 public immutable FUNCTION_ID;

    constructor(address _functionGateway, bytes32 _functionId) {
        FUNCTION_GATEWAY = _functionGateway;
        FUNCTION_ID = _functionId;
    }

    function set(uint256 _a) external {
        a = _a;
    }

    function mirrorStorageSlot(address addr, bytes32 location) external {
        bytes32 l1_hash = IL1Block(address(0x42)).hash();
        uint256 l1_number = IL1Block(address(0x42)).number();
        bytes32 storageHash = keccak256(abi.encodePacked(addr, location));

        IFunctionGateway(FUNCTION_GATEWAY).request(
            FUNCTION_ID, 
            abi.encodePacked(l1_hash, addr, location),
            this.storeSlotValue.selector,
            abi.encode(l1_number,storageHash)
        );
    }

    function storeSlotValue(bytes memory _output, bytes memory _context) external {
        require(msg.sender == FUNCTION_GATEWAY, "Only function gateway can call this function");

        (uint256 l1_number, bytes32 storageHash) = abi.decode(_context, (uint256));
        (, bytes32 slotValue) = OutputReader.readBytes32(_output, 0);
        storageSlotToValue[storageHash] = slotValue;
        lastUpdated[storageHash] = l1_number;
    }
}