#!/bin/bash
# Generate 3 key shards for local threshold signatures
set -e

echo "=== CHAKRA KEY SHARD GENERATOR ==="
echo "Generating new secp256k1 key splits..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed. Please install Rust."
    exit 1
fi

# Run the test binary or custom main tool to produce shards
cd "$(dirname "$0")/.."
cargo run --bin chakra-sentinel -- --mode keygen

echo "Shards generated: shard_1.json, shard_2.json, shard_3.json"
echo "Distribute these files to your nodes."
