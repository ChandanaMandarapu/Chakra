# Contributing to CHAKRA

Thank you for your interest in contributing to CHAKRA — the cross-chain execution protocol built on Solana.

CHAKRA is currently in active early development. The core protocol is deployed on Solana Devnet and we are working toward Milestone 2: a live, publicly verifiable bridgeless transaction from Solana to Base Sepolia.

---

## Ways to Contribute

### 1. Report Bugs

If you find a bug in the Anchor program, Sentinel Node, or test suite:

- Open an issue on GitHub with the title format: `[BUG] Short description`
- Include the exact command you ran
- Include the full error output
- Include your Rust/Anchor/Node.js versions
- Include which network you were using (localnet / devnet)

### 2. Report Security Vulnerabilities

**Do not open a public issue for security vulnerabilities.**  
See [SECURITY.md](SECURITY.md) for the responsible disclosure process.

### 3. Suggest Features

Open an issue with the title format: `[FEATURE] Short description`  
Describe what you want to build, why it matters for cross-chain execution, and how it fits the CHAKRA architecture.

### 4. Submit Code

We welcome pull requests for:

- Bug fixes in the Anchor program or Sentinel Network
- Additional test coverage
- Documentation improvements
- ChakraReceiver.sol improvements
- Sentinel Node networking improvements

**Before submitting a PR:**

1. Fork the repository
2. Create a branch: `git checkout -b fix/your-description`
3. Make your changes
4. Run the test suite and confirm all 4 tests pass:
   ```bash
   cd chakra-onchain
   anchor test --provider.cluster devnet --skip-deploy
   ```
5. Run Rust formatting and linting:
   ```bash
   cd chakra-sentinel
   cargo fmt --all
   cargo clippy -- -D warnings
   ```
6. Write a clear PR description explaining what changed and why

---

## Development Setup

### Requirements

| Tool | Version |
|------|---------|
| Rust | 1.85.0 |
| Solana CLI | Latest stable |
| Anchor | 0.32.1 |
| Node.js | 18+ |
| Yarn | 1.22+ |

### Setup Steps

```bash
# 1. Clone
git clone https://github.com/ChandanaMandarapu/chakra
cd chakra

# 2. Configure Solana for devnet
solana config set --url devnet

# 3. Create a keypair if you don't have one
solana-keygen new --outfile ~/.config/solana/id.json

# 4. Get devnet SOL
solana airdrop 2

# 5. Build the Anchor program
cd chakra-onchain
anchor build

# 6. Run tests
anchor test --provider.cluster devnet --skip-deploy

# 7. Build the Sentinel
cd ../chakra-sentinel
cargo build
```

---

## Code Style

### Rust (Anchor Program + Sentinel)

- Follow standard Rust idioms
- Use `anyhow::Result` for error handling in the Sentinel
- Use Anchor error codes (in `errors.rs`) for on-chain errors — never panic in program code
- Every public function should have a doc comment explaining what it does
- Keep instruction handlers in separate files under `instructions/`

### TypeScript (Tests)

- Use `async/await` — no `.then()` chains
- Every test should have a clear comment explaining what invariant it's testing
- Use `assert` from `chai` for all assertions

### Solidity (ChakraReceiver)

- Follow the Solidity style guide
- Every state-changing function must emit an event
- Replay protection is mandatory for any function that processes cross-chain messages

---

## Architecture Decisions

Before proposing a significant change, please understand these core decisions:

**Why Shamir Secret Sharing instead of true MPC-ECDSA?**  
SSS with partial secp256k1 signing is the current Milestone 1 implementation. It is a deliberate stepping stone. True FROST MPC-ECDSA (where the master key never exists in any single location, even at generation time) is the Milestone 2 upgrade. The current approach proves the full flow while the more complex MPC protocol is implemented correctly.

**Why Solana as the command layer?**  
65,000 TPS. 150ms finality. Sub-cent transaction costs. Native secp256k1 recovery syscall. Anchor for safe program development. Solana is the only chain fast and cheap enough to be a universal execution coordinator.

**Why atomic escrow instead of optimistic execution?**  
Cross-chain atomicity is the hardest problem in distributed systems. The escrow model — lock first, execute, prove, release — gives users a hard guarantee: their funds either move completely or return completely. There is no partial state.

---

## Questions

Open a discussion on GitHub or reach out on X: [@chandana_mndrpu](https://x.com/chandana_mndrpu)