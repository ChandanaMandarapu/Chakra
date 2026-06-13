# CHAKRA Anchor Integration Tests Spec

This document describes the testing scenarios covered by the Anchor TypeScript integration test suite.

---

## 1. Test Setup
*   **Provider Cluster:** Local validator or Devnet.
*   **Authority Accounts:** The deployer functions as the default admin and initializes the global state config.

---

## 2. Test Scenarios

### 2.1 Test Case: `Initialize Config`
*   **Input:** Treasury public key.
*   **Assertion:** Global config PDA matches expected values, setting `is_initialized = true` and registering the correct treasury.

### 2.2 Test Case: `Initialize Intent (Lock Escrow)`
*   **Input:** Destination chain ID, amount (in lamports), unique nonce, target wallet address.
*   **Assertion:** 
    *   System transfers specified lamports from User to Escrow PDA.
    *   Escrow PDA contains the correct initialization state.
    *   `ControlIntent` event is emitted.

### 2.3 Test Case: `Submit Proof (Unlock Escrow)`
*   **Input:** TSS signature matching the registered key.
*   **Assertion:** 
    *   Signature successfully verified.
    *   Escrow PDA closed, transferring all locked lamports to the Treasury.
    *   `IntentFinalized` event is emitted.
