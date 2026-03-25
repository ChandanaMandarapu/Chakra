# CHAKRA — The Universal Mainframe

**Turn Solana into the command layer for all blockchains.**

Live Demo: https://chakra-mainframe.vercel.app/  
Technical Demo: https://youtu.be/hDFFVkvAfuM  

## Architecture

CHAKRA uses Distributed Key Generation (DKG) and 2-of-3 Threshold 
Signature Schemes (TSS) to let Solana programs control native 
accounts on Bitcoin, Ethereum, and Base—no bridges, no wrapped assets.

### Components
- **CHAKRA Controller**: Anchor program on Solana
- **Sentinel Network**: Decentralized TSS nodes (8GB RAM consumer hardware)
- **Atomic Escrow**: Time-locked with ZK-rollback guarantees
- **Universal SDK**: Rust crate + TypeScript client

## Status

- ✅ Milestone 0: Live landing page + WebGL visualization
- 🔄 Milestone 1: TSS implementation (in development)
- ⏳ Milestone 2: Universal SDK

## Grant

Solana Foundation Developer Tooling Grant applicant.
