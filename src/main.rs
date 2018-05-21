extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::time::{SystemTime, UNIX_EPOCH};

struct Block {
    hash: String,
    pub previous_hash: String,
    data: String,
    timestamp: u64
}

impl Block {
    pub fn new(data: &str, previous_hash: &str) -> Self {
        
        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

        let timestamp = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;

        let hash = Self::calculate_hash(previous_hash, timestamp, data);

        Self {
            data: data.to_string(),
            hash,
            previous_hash: previous_hash.to_string(),
            timestamp 
        }
    }

    pub fn calculate_hash(previous_hash: &str, timestamp: u64, data: &str) -> String {
        let mut sha = Sha256::new();
        let input = format!("{}{}{}", previous_hash, timestamp, data);
        sha.input_str(&input);
        let hash = sha.result_str();

        hash
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }
}

pub type Blockchain = Vec<Block>;

impl Blockchain {

}

fn main() {
    let mut blockchain = Vec::new();

    blockchain.push(Block::new("I AM THE GENESIS BLOCK, THE ONE AND ONLY.", "0"));
    blockchain.push(Block::new("Second block mofos", 
        blockchain.last().unwrap().hash());
    blockchain.push(Block::new("Third block up in the house", 
        blockchain.last().unwrap().hash());

}
