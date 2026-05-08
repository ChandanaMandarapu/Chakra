# CHAKRA — The Universal Cross-Chain Mainframe

**Turn Solana into the trustless command layer for all blockchains.**

CHAKRA is a bridgeless cross-chain execution protocol that uses Distributed Key Generation (DKG) and Threshold Signature Schemes (TSS) to let Solana smart contracts directly own and control native accounts on other chains (such as Bitcoin, Ethereum, and Base).

*   **Vercel Live Demo:** [chakra-mainframe.vercel.app](https://chakra-mainframe.vercel.app/)
*   **Video Walkthrough:** [YouTube Link](https://youtu.be/hDFFVkvAfuM)

---

## The Vision: Solana as the Mainframe

In the multi-chain world, blockchains are isolated islands. Moving assets usually requires "bridges"—trusted intermediaries that lock assets in custody vaults and mint wrapped IOUs. These vaults are giant honeypots, resulting in over $2 billion in hacks.

CHAKRA eliminates the honeypot entirely. Instead of moving assets between chains:
1.  **Decentralized Sentinel Nodes** run a threshold signature scheme (TSS) to manage a split private key.
2.  The **Solana Anchor Program** acts as the execution coordinator.
3.  Sentinel nodes monitor Solana for user intents, sign the execution proofs via Shamir Secret Sharing, and coordinate execution on the destination chain (e.g., EVM).
4.  **No wrapped assets. No custody contract honeypot. Pure cryptographic coordination.**

---

## Component Architecture

CHAKRA consists of three primary codebases:

```
                  ┌────────────────────────┐
                  │   User Init Escrow     │
                  └──────────┬─────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────┐
│              chakra-onchain (Solana Program)            │
│  - Locks User Funds in PDA Escrow                       │
│  - Emits ControlIntent event                            │
│  - Verifies signatures via secp256k1 recover            │
└────────────┬──────────────────────────────▲─────────────┘
             │                              │
             │ WebSockets                   │ submit_proof
             ▼                              │
┌───────────────────────────────────────────┴─────────────┐
│              chakra-sentinel (Rust Node)                │
│  - Monitors Solana for incoming execution events        │
│  - Coordinates 3 nodes holding 2-of-3 key shards        │
│  - Reconstructs and signs intent payload               │
└────────────┬────────────────────────────────────────────┘
             │
             │ Execute Tx
             ▼
┌─────────────────────────────────────────────────────────┐
│               chakra-receiver (EVM side)                │
│  - Verifies TSS signature on target chain               │
│  - Releases native assets or triggers execution         │
└─────────────────────────────────────────────────────────┘
```

1.  **`chakra-onchain` (Solana Program)**
    *   Written in Rust using the Anchor framework.
    *   Locks user assets in an atomic program-derived address (PDA) escrow.
    *   Verifies threshold signatures on-chain via the native `secp256k1_recover` syscall.
    *   Handles timeout-based cancellation, providing mathematical rollback guarantees.

2.  **`chakra-sentinel` (TSS Network)**
    *   A distributed Rust service running Axum-based signing servers.
    *   Uses 2-of-3 threshold cryptography (Shamir Secret Sharing) to reconstruct keys and produce signatures.
    *   Contains a coordinator that listens to Solana via WebSocket subscription, communicates with sentinel nodes to compile signature proofs, and executes them on EVM targets.

3.  **`chakra-web` (Frontend Dashboard)**
    *   A React/TypeScript web app built with Vite.
    *   Provides a sleek interface for users to lock funds and trigger cross-chain executions.
    *   Displays real-time transaction progress and sentinel network status.

---

## Repository Structure

```
├── chakra-onchain/       # Solana Anchor program & EVM contracts
│   ├── programs/         # Solana smart contract logic
│   ├── src/              # Solidity receiver contracts
│   └── tests/            # TypeScript integration test suite
├── chakra-sentinel/      # Rust sentinel nodes & signing coordinator
│   ├── src/              # Axum nodes, event listener, & signer logic
│   └── scripts/          # Node management scripts
├── chakra-web/           # Frontend dashboard code
└── docs/                 # In-depth design & technical specs
```

---

## License

This project is open-source and licensed under the MIT License. See [LICENSE](LICENSE) for details.
