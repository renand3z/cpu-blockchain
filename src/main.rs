use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: &str, previous_hash: &str) -> Block {
        let hash = calculate_hash(index, data, previous_hash);

        Block {
            index,
            data: data.to_string(),
            previous_hash: previous_hash.to_string(),
            hash,
        }
    }
}

fn calculate_hash(index: u64, data: &str, previous_hash: &str) -> String {
    let data = format!("{}{}{}", index, data, previous_hash);
    let mut sha256 = Sha256::new();
    sha256.input_str(&data);
    sha256.result_str()
}

fn main() {
    let mut blockchain = vec![Block::new(0, "Genesis Block", "")];

    loop {
        let data = get_user_input("Enter data for the next block: ");
        let index = blockchain.len() as u64;
        let previous_hash = &blockchain[index as usize - 1].hash;
        let new_block = Block::new(index, &data, previous_hash);

        blockchain.push(new_block.clone());
        println!("New block added: {:?}", new_block);
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