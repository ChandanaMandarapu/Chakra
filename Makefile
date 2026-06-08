# CHAKRA Developer Commands

.PHONY: test-onchain test-sentinel run-node-1 run-node-2 run-node-3 run-listener clean

test-onchain:
	cd chakra-onchain && anchor test

test-sentinel:
	cd chakra-sentinel && cargo test

run-node-1:
	cd chakra-sentinel && cargo run -- --shard shard_1.json --port 3001 --mode node

run-node-2:
	cd chakra-sentinel && cargo run -- --shard shard_2.json --port 3002 --mode node

run-node-3:
	cd chakra-sentinel && cargo run -- --shard shard_3.json --port 3003 --mode node

run-listener:
	cd chakra-sentinel && cargo run -- --shard shard_1.json --mode listener

clean:
	cd chakra-onchain && anchor clean
	cd chakra-sentinel && cargo clean
