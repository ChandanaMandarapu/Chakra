import json
import secrets

def generate_mock_shards():
    # In a real DKG, this is distributed. 
    # For Milestone 1, we generate and split locally to simulate the 2-of-3 setup.
    shards = []
    for i in range(1, 4):
        # Generating 32-byte hex values for the shards
        shard_val = secrets.token_hex(32)
        shards.append({
            "index": i,
            "value": shard_val
        })
    
    for i, shard in enumerate(shards):
        filename = f"chakra-sentinel/shard_{i+1}.json"
        with open(filename, "w") as f:
            json.dump(shard, f, indent=4)
        print(f"Generated {filename}")

if __name__ == "__main__":
    generate_mock_shards()
