# CHAKRA — Cryptographic Technical Specification
*Threshold Cryptography, Shamir Secret Sharing, and On-Chain Verification*

---

## 1. Introduction to Threshold Cryptography (2-of-3)

CHAKRA uses a Threshold Signature Scheme (TSS) to decentralize the private key that controls the target EVM wallet. Instead of storing the private key on a single server, the key is split into $N=3$ secret shards. Any $T=2$ shards are mathematically sufficient to produce a valid signature.

A 2-of-3 threshold is chosen for:
1.  **Liveness:** Any single node can go offline, and the network can still sign transactions.
2.  **Safety:** An attacker must compromise at least two independent nodes to reconstruct the key or forge a signature.

---

## 2. Mathematical Foundation: Shamir Secret Sharing

Shamir's Secret Sharing (SSS) is based on the algebraic property that $K$ points uniquely define a polynomial of degree $K-1$. For a 2-of-3 threshold ($T=2$), we use a polynomial of degree $T-1 = 1$ (a straight line):

$$f(x) = a_0 + a_1 x \pmod{p}$$

Where:
*   $a_0$ is the secret (the master private key).
*   $a_1$ is a random coefficient generated over the scalar field of the elliptic curve.
*   $p$ is the prime order of the secp256k1 curve group:
    $$p = 2^{256} - 2^{32} - 977$$
    In hexadecimal representation, this is:
    `FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141`

### Shard Generation
At key generation time:
1.  A coordinator or DKG protocol generates the master private key $a_0$ and a random $a_1$.
2.  Shards are evaluated at integer coordinates $x \in \{1, 2, 3\}$:
    *   $\text{Shard}_1 = (1, f(1)) = (1, a_0 + a_1 \pmod{p})$
    *   $\text{Shard}_2 = (2, f(2)) = (2, a_0 + 2a_1 \pmod{p})$
    *   $\text{Shard}_3 = (3, f(3)) = (3, a_0 + 3a_1 \pmod{p})$
3.  Each node receives exactly one shard (index and value) via secure transport.

### Reconstruction via Lagrange Interpolation
To sign a message or reconstruct the master key $a_0 = f(0)$ using any two shards (say, node $i$ and node $j$):

$$f(0) = \sum_{m \in \{i, j\}} y_m \cdot \ell_m(0) \pmod{p}$$

Where the Lagrange basis polynomials evaluated at $x=0$ are:

$$\ell_i(0) = \frac{0 - x_j}{x_i - x_j} = \frac{x_j}{x_j - x_i} \pmod{p}$$
$$\ell_j(0) = \frac{0 - x_i}{x_j - x_i} = \frac{x_i}{x_i - x_j} \pmod{p}$$

These coefficients $\ell_i(0)$ and $\ell_j(0)$ are calculated mod $p$ using modular inverse (Fermat's Little Theorem: $a^{-1} \equiv a^{p-2} \pmod{p}$).

---

## 3. Signature Generation and Verification Flow

### Milestone 1 Key Reconstruction Model
1.  **Intent Detected:** The coordinator hears a `ControlIntent` event from Solana.
2.  **Request Shards:** The coordinator queries 2 out of 3 Sentinel Nodes for their key shards.
3.  **Local Reconstruction & Signing:** The coordinator reconstructs the private key $a_0$ temporarily in memory, signs the transaction message hash, and wipes the key from memory.
4.  **On-Chain Verification:** The signature is submitted back to the Solana program.

### On-Chain Verification via `secp256k1_recover`
Solana provides a native syscall `secp256k1_recover` to recover the 64-byte public key of a signer from an Ethereum-style signature:
*   Input: `hash` (32 bytes), `recovery_id` (0-3), `signature` (64 bytes).
*   Output: `public_key` (64 bytes, uncompressed, without the `0x04` prefix).

The Anchor program maintains a `TssConfig` PDA storing the expected uncompressed `tss_pubkey`. If the recovered public key matches the registered public key, the proof is accepted.

---

## 4. Upgrade Path to FROST (Milestone 2)

In Milestone 1, the coordinator acts as a trusted compiler that temporarily reconstructs the key in memory to sign. While simple, this creates a single point of failure during signing.

In Milestone 2, CHAKRA will transition to **FROST (Flexible Round-Optimized Threshold Signatures)**:
*   **Zero Key Reconstruction:** Nodes communicate via a two-round protocol to generate partial signature shares.
*   **Joint Signature Creation:** The shares are aggregated into a single standard secp256k1 signature without ever combining the private key shards.
*   **Security:** Even if the coordinator is compromised, the master private key remains completely safe.
