// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract ChakraReceiver {
    using ECDSA for bytes32;

    address public protocol_public_key;

    constructor(address _protocol_key) {
        protocol_public_key = _protocol_key;
    }

    // Verify the TSS signature from the Sentinel Nodes
    function validate_intent(
        uint256 amount,
        address target,
        bytes32 r,
        bytes32 s,
        uint8 v
    ) public view returns (bool) {
        bytes32 messageHash = keccak256(abi.encodePacked("transfer:", amount, ":base:", target));
        bytes32 ethSignedMessageHash = messageHash.toEthSignedMessageHash();
        
        address signer = ecrecover(ethSignedMessageHash, v, r, s);
        return (signer == protocol_public_key);
    }
}
