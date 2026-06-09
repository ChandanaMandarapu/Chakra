# Security Policy

CHAKRA is a cross-chain execution protocol handling real cryptographic keys and user funds on Solana Devnet. I take security seriously and appreciate responsible disclosure from the security community.

---

## Scope

The following components are in scope for security reports:

| Component | Location | Risk Level |
|-----------|----------|------------|
| CHAKRA Controller (Anchor program) | `chakra-onchain/programs/` | Critical |
| Sentinel Node signing logic | `chakra-sentinel/src/signer.rs` | Critical |
| ChakraReceiver.sol | `chakra-onchain/src/ChakraReceiver.sol` | High |
| Sentinel Node HTTP server | `chakra-sentinel/src/node_server.rs` | High |
| Key shard generation | `chakra-sentinel/src/signer.rs` | Critical |
| Test suite (informational only) | `chakra-onchain/tests/` | Low |

---

## Current Deployment

CHAKRA is currently deployed on **Solana Devnet only**. No real user funds are at risk during this phase. However, we treat all vulnerabilities seriously because:

1. The architecture being built now will protect real funds on mainnet
2. Cryptographic weaknesses found now are far less costly to fix than after mainnet deployment
3. Responsible disclosure helps us build better security practices from day one

---

## What We Want to Hear About

**Critical — please report immediately:**
- Any way to drain the escrow PDA without a valid TSS proof
- Any way to forge a valid TSS signature without controlling 2+ Sentinel Node shards
- Any way to bypass the `is_finalized` or `is_cancelled` checks in `submit_proof` or `cancel_intent`
- Any replay attack vector in `ChakraReceiver.sol` that bypasses the `executed` mapping
- Any way to reconstruct the master private key from a single shard

**High — please report:**
- Logic errors in the Lagrange interpolation in `combine_signatures`
- Timing attacks against the Sentinel Node HTTP server
- Any way to cause `cancel_intent` to succeed before the timeout has passed
- Integer overflow or underflow in the Anchor program that bypasses safety checks

**Medium — please report:**
- Denial-of-service vectors against the Sentinel Node network
- Information leakage from the Sentinel Node HTTP API
- Issues with the escrow PDA seed derivation

---

## What Is Not In Scope

- Issues in dependencies that have existing CVEs (report to upstream)
- Social engineering attacks
- Issues requiring physical access to Sentinel Node hardware
- Theoretical weaknesses without a proof-of-concept

---

## How to Report

**Do not open a public GitHub issue for security vulnerabilities.**

Please report security vulnerabilities by sending an email to:

**security@chakraprotocol.dev** *(check GitHub for current contact)*

Or reach out directly on X: [@chandana_mndrpu](https://x.com/chandana_mndrpu)

Your report should include:

1. A clear description of the vulnerability
2. The affected component and file(s)
3. Step-by-step reproduction instructions
4. The impact you believe this has on user funds or key security
5. If possible, a proof-of-concept (even pseudocode is helpful)

---

## Response Process

| Step | Timeline |
|------|----------|
| Acknowledgment of your report | Within 48 hours |
| Initial assessment and severity classification | Within 5 days |
| Fix development and testing | Depends on severity (critical: 7 days, high: 14 days) |
| Public disclosure (coordinated with reporter) | After fix is deployed |

---

## Disclosure Policy

I follow coordinated disclosure. We ask that you:

- Give me reasonable time to fix the issue before public disclosure
- Not exploit the vulnerability or access data beyond what is necessary to prove the issue
- Not disrupt the Devnet deployment or other users' testing

In return, I will:

- Acknowledge your contribution publicly (if you want)
- Keep you informed of my progress
- Work with you on the disclosure timeline

---

## Known Limitations (Not Vulnerabilities)

These are known architectural decisions that will be upgraded in future milestones:

## Thank You

Security researchers who responsibly disclose valid vulnerabilities will be credited in the project's security acknowledgments. Your work makes CHAKRA safer for everyone who builds on it.