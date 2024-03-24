# CPU Blockchain

This is a simple example implementation of a blockchain that uses CPU cycles to count time. 

## Features

- Create new blocks with user-provided data
- Calculate the hash of each block using the SHA-256 algorithm
- Use normalized CPU cycles as a "timestamp" for each block
- Display the newly added block after creation

## Usage

1. Build and run the project:

```
cargo run
```

2. Enter data for the next block when prompted:

```
Enter data for the next block: Hello, Blockchain!
```

The program will output the new block details:

```
New block added: Block { index: 1, data: "Hello, Blockchain!", previous_hash: "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb", hash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", normalized_cycles: 123456789 }
```

3. Continue entering data for new blocks or press `Ctrl+C` to exit.

## Implementation Details

The `Block` struct represents a single block in the blockchain, containing the following fields:

- `index`: The index of the block in the blockchain
- `data`: The data stored in the block
- `previous_hash`: The hash of the previous block
- `hash`: The hash of the current block
- `normalized_cycles`: The "timestamp" based on normalized CPU cycles

The `calculate_hash` function computes the SHA-256 hash of a block using its index, data, previous hash, and normalized CPU cycles.

The `main` function initializes the blockchain with a genesis block and then enters a loop to continuously add new blocks with user-provided data. The "timestamp" for each block is calculated using the elapsed CPU cycles since the program started, normalized to a reference clock rate of 2.5 GHz.