use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::Instant;

const REFERENCE_CLOCK_RATE: f64 = 2500000000.0; // 2.5 GHz

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    data: String,
    previous_hash: String,
    hash: String,
    normalized_cycles: u128, // Using normalized CPU cycles as the "timestamp"
}

impl Block {
    fn new(index: u64, data: &str, previous_hash: &str, normalized_cycles: u128) -> Block {
        let hash = calculate_hash(index, data, previous_hash, normalized_cycles);

        Block {
            index,
            data: data.to_string(),
            previous_hash: previous_hash.to_string(),
            hash,
            normalized_cycles,
        }
    }
}

fn calculate_hash(index: u64, data: &str, previous_hash: &str, normalized_cycles: u128) -> String {
    let data = format!("{}{}{}{}", index, data, previous_hash, normalized_cycles);
    let mut sha256 = Sha256::new();
    sha256.input_str(&data);
    sha256.result_str()
}

fn main() {
    let mut blockchain = vec![Block::new(0, "Genesis Block", "", 0)];
    let mut start_time = Instant::now();
    let cpu_freq = cpu_frequency();

    loop {
        let data = get_user_input("Enter data for the next block: ");
        let index = blockchain.len() as u64;
        let previous_hash = &blockchain[index as usize - 1].hash;
        let elapsed_cycles = (start_time.elapsed().as_nanos() as f64 * cpu_freq / 1000000000.0) as u128;
        let normalized_cycles = (elapsed_cycles as f64 * REFERENCE_CLOCK_RATE / cpu_freq) as u128;
        let new_block = Block::new(index, &data, previous_hash, normalized_cycles);

        blockchain.push(new_block.clone());
        println!("New block added: {:?}", new_block);
        start_time = Instant::now();
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn cpu_frequency() -> f64 {
    // Implementation to detect the CPU frequency
    // You can use platform-specific APIs or benchmarking techniques
    // For simplicity, we'll assume a fixed CPU frequency of 2.5 GHz
    2500000000.0
}