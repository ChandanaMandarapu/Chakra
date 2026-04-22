// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

contract ChakraReceiver {
    using ECDSA for bytes32;
    using MessageHashUtils for bytes32;

    address public protocol_public_key;
    mapping(bytes32 => bool) public executed;

    event IntentExecuted(bytes32 indexed intentId, address target, uint256 amount);

    constructor(address _protocol_key) {
        protocol_public_key = _protocol_key;
    }

    // Verify and execute the cross-chain intent
    function execute_intent(
        uint64 target_chain_id,
        uint256 amount,
        address target,
        uint256 nonce,
        bytes32 r,
        bytes32 s,
        uint8 v
    ) public {
        // Construct the unique intent ID (nonce + target + amount)
        bytes32 intentId = keccak256(abi.encodePacked(nonce, target, amount));
        
        require(!executed[intentId], "Chakra: Intent already executed");
        
        // Pad target address to 64 bytes to match Solana signing format
        // (20 bytes address + 44 bytes zeros)
        bytes memory paddedTarget = abi.encodePacked(target, new bytes(44));
        
        // Match the Sentinel's binary signing format exactly:
        // [target_chain_id (8), nonce (8), amount (8), target_address (64)]
        bytes32 messageHash = keccak256(abi.encodePacked(
            target_chain_id,  
            uint64(nonce),    
            uint64(amount),   
            paddedTarget      
        ));
        
        bytes32 ethSignedMessageHash = messageHash.toEthSignedMessageHash();
        
        address signer = ECDSA.recover(ethSignedMessageHash, v, r, s);
        require(signer == protocol_public_key, "Chakra: Invalid TSS signature");

        executed[intentId] = true;
        
        // Effectuate the transfer (e.g., direct release of native asset)
        payable(target).transfer(amount); 

        emit IntentExecuted(intentId, target, amount);
    }
}
