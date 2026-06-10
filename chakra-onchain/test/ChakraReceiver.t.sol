// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/ChakraReceiver.sol";

contract ChakraReceiverTest is Test {
    ChakraReceiver public receiver;
    uint256 public privateKey = 0xA1B2C3D4E5;
    address public protocolKey;

    function setUp() public {
        protocolKey = vm.addr(privateKey);
        receiver = new ChakraReceiver(protocolKey);
        vm.deal(address(receiver), 100 ether);
    }

    function testExecuteIntent() public {
        uint64 chainId = 1;
        uint256 amount = 1 ether;
        address target = address(0x123);
        uint256 nonce = 0;

        // Build target padding to 64 bytes
        bytes memory paddedTarget = abi.encodePacked(target, new bytes(44));

        // Construct message hash
        bytes32 messageHash = keccak256(abi.encodePacked(
            chainId,
            uint64(nonce),
            uint64(amount),
            paddedTarget
        ));

        // Sign the hash
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, messageHash);

        uint256 balanceBefore = target.balance;

        // Execute intent
        receiver.execute_intent(chainId, amount, target, nonce, r, s, v);

        assertEq(target.balance, balanceBefore + amount);
        assertTrue(receiver.executed(keccak256(abi.encodePacked(nonce, target, amount))));
    }
}
