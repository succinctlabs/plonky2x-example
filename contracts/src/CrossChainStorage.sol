// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

// import {IFunctionGateway} from "@succinct-sdk/interfaces/IFunctionGateway.sol";
// import {OutputReader} from "@succinct-sdk/libraries/OutputReader.sol";

// The interface for the Optimism L1 block contract: https://github.com/ethereum-optimism/optimism/blob/develop/packages/contracts-bedrock/src/L2/L1Block.sol
interface L1Block {
    function hash() external view returns (bytes32);

    function number() external view returns (uint64);
}

interface IFunctionGateway {
    function request(
        bytes32 functionId,
        bytes memory inputs,
        bytes4 select,
        bytes memory context
    ) external payable;
}

contract CrossChainStorageSlot {
    address public constant FUNCTION_GATEWAY =
        0x852a94F8309D445D27222eDb1E92A4E83DdDd2a8;
    bytes32 public constant FUNCTION_ID =
        0x2b6431895aa4eabb46c3416c1c6c9ebf1ea06923fd68e70ef6c7349d1254ecf6;
    address immutable L1_BLOCK;

    mapping(address => mapping(bytes32 => bytes32))
        public addressToLocationToValue;
    mapping(address => mapping(bytes32 => uint64))
        public addressToLocationToLastUpdated;

    event StorageSlotMirrored(
        address addr,
        bytes32 location,
        bytes32 value,
        uint256 blockNumber
    );

    constructor(address l1block) {
        L1_BLOCK = l1block;
    }

    // TODO: replace this with the Succinct library
    function readBytes32(bytes memory _output) internal pure returns (bytes32) {
        bytes32 value;
        assembly {
            value := mload(add(_output, 0x20))
        }
        return value;
    }

    function requestMirrorSlot(
        address addr,
        bytes32 location
    ) external payable {
        require(msg.value >= 30 gwei * 1_000_000); // 1_000_000 is the default gas limit

        bytes32 l1_hash = L1Block(L1_BLOCK).hash();
        uint64 l1_number = L1Block(L1_BLOCK).number();

        IFunctionGateway(FUNCTION_GATEWAY).request{value: msg.value}(
            FUNCTION_ID,
            abi.encodePacked(l1_hash, addr, location),
            this.storeSlotValue.selector,
            abi.encode(l1_number, addr, location)
        );
    }

    function storeSlotValue(
        bytes memory _output,
        bytes memory _context
    ) external {
        require(
            msg.sender == FUNCTION_GATEWAY,
            "Only function gateway can call this function"
        );

        (uint64 l1_number, address addr, bytes32 location) = abi.decode(
            _context,
            (uint64, address, bytes32)
        );
        bytes32 value = readBytes32(_output);

        addressToLocationToValue[addr][location] = value;
        addressToLocationToLastUpdated[addr][location] = l1_number;
        emit StorageSlotMirrored(addr, location, value, l1_number);
    }
}
