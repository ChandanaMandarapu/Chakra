# CHAKRA Technical Glossary

This glossary defines terms and abbreviations used throughout the CHAKRA mainframe documentation.

---

*   **TSS (Threshold Signature Scheme):** A cryptographic protocol that enables a set of parties to joint-sign a message if a threshold $T$ of players participate.
*   **DKG (Distributed Key Generation):** A process where multiple parties participate in calculating a shared public and private key set without any single party learning the private key.
*   **SSS (Shamir's Secret Sharing):** An algorithm where a secret is split into unique parts (shards), requiring a minimum threshold of parts to reconstruct the original secret.
*   **PDA (Program Derived Address):** A unique account on Solana that is controlled by a specific program rather than a private key, enabling programs to safely escrow tokens.
*   **Sentinel Node:** A validator node in the CHAKRA network holding a single private key shard and running an HTTP signing service.
*   **Coordinator:** The node in the Sentinel network responsible for listening to Solana, requesting signature shares from peers, combining them, and executing target transactions.
*   **Solana Mainframe:** The concept of using Solana's high speed and low cost to coordinate execution logic across all other blockchains.
