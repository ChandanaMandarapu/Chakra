# Local Development Setup Guide
*Spin up the CHAKRA local mainframe and sentinel network in 5 minutes*

This guide walks you through deploying the CHAKRA Anchor program to a local validator and starting up a 3-node Sentinel signature network locally.

---

## 1. Prerequisites

Ensure you have the following installed on your system:
*   **Rust & Cargo:** (v1.85.0+)
*   **Solana CLI:** Stable release
*   **Anchor CLI:** (v0.32.1+)
*   **Node.js & Yarn**

---

## 2. Step-by-Step Installation

### Step 2.1: Clone and Configure
```bash
# Clone the repository
git clone https://github.com/ChandanaMandarapu/chakra.git
cd chakra

# Point Solana CLI to localhost
solana config set --url localhost
```

### Step 2.2: Compile and Deploy the Anchor Program
```bash
cd chakra-onchain

# Build the Anchor program
anchor build

# Start a local validator in a separate terminal
solana-test-validator

# Deploy the program to localnet
anchor deploy
```

### Step 2.3: Set Up the Sentinel Environment
Configure the environment variables by duplicating the example file:
```bash
cd ../chakra-sentinel
cp .env.example .env
```

### Step 2.4: Start the Sentinel Nodes
To simulate a decentralized 2-of-3 threshold network, we spin up 3 nodes listening on ports `3001`, `3002`, and `3003`:

```bash
# Terminal 1 - Node 1
cargo run -- --shard shard_1.json --port 3001 --mode node

# Terminal 2 - Node 2
cargo run -- --shard shard_2.json --port 3002 --mode node

# Terminal 3 - Node 3
cargo run -- --shard shard_3.json --port 3003 --mode node
```

### Step 2.5: Run the Listener/Coordinator
The coordinator monitors Solana for `ControlIntent` events and orchestrates the TSS signature gathering from the nodes:

```bash
# Terminal 4 - Start Listener Coordinator
cargo run -- --shard shard_1.json --mode listener
```

---

## 3. Running a Local Simulation

To verify everything is working end-to-end, run the Anchor integration tests:
```bash
cd ../chakra-onchain
anchor test --provider.cluster localnet
```
If successful, the test suite will:
1.  Initialize global configs and the TSS signature registry.
2.  Lock a user's funds in an Escrow PDA.
3.  Simulate sentinel signature collection and submit the combined proof back to Solana, successfully releasing the funds to the treasury wallet.
