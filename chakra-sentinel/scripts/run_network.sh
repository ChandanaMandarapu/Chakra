#!/bin/bash
# Spin up 3 Sentinel Nodes locally for threshold signing simulation
set -e

echo "=== STARTING CHAKRA SENTINEL NETWORK ==="

cd "$(dirname "$0")/.."

echo "Starting Node 1 on port 3001..."
cargo run -- --shard shard_1.json --port 3001 --mode node &
PID1=$!

echo "Starting Node 2 on port 3002..."
cargo run -- --shard shard_2.json --port 3002 --mode node &
PID2=$!

echo "Starting Node 3 on port 3003..."
cargo run -- --shard shard_3.json --port 3003 --mode node &
PID3=$!

echo "Network running. PIDs: $PID1, $PID2, $PID3"
echo "Press Ctrl+C to stop all nodes."

trap "kill $PID1 $PID2 $PID3; exit" INT
wait
