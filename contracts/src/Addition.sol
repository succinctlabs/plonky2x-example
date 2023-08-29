// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {IFunctionGateway} from "succinctx/interfaces/IFunctionGateway.sol";
import {OutputReader} from "succinctx/libraries/OutputReader.sol";

contract SimpleCircuit {
    uint256 public nextRequestId = 1;
    address public constant FUNCTION_GATEWAY = 0x852a94F8309D445D27222eDb1E92A4E83DdDd2a8;
    bytes32 public constant FUNCTION_ID = 0xe58700a5c991de3a6032fcd489d8711341dda3ae776aff1c163315ffbf7fd92b;

    event CallbackReceived(uint256 requestId, uint32 a_plus_b);

    function requestAddition(uint32 a, uint32 b) external payable {
        require(msg.value >= 30 gwei * 1_000_000); // 1_000_000 is the default gas limit
        IFunctionGateway(FUNCTION_GATEWAY).request{value: msg.value}(
            FUNCTION_ID, abi.encodePacked(a, b), this.handleCallback.selector, abi.encode(nextRequestId)
        );
        nextRequestId++;
    }

    function handleCallback(bytes memory output, bytes memory context) external {
        require(msg.sender == FUNCTION_GATEWAY);
        uint256 requestId = abi.decode(context, (uint256));
        (, uint32 a_plus_b) = OutputReader.readUint32(output, 0);
        emit CallbackReceived(requestId, a_plus_b);
    }
}
