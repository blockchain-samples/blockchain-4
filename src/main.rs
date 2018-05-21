extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

struct Block {
    hash: String,
    pub previous_hash: String,
    pub data: String,
    pub timestamp: u64
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

struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new()
        }
    }

    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn last(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = self.blocks.get(i).expect("No blocks found");
            let previous_block = self.blocks.get(i - 1).expect("Need at least 2 blocks");

            if current_block.hash() != Block::calculate_hash(&current_block.previous_hash, current_block.timestamp, &current_block.data) {
                println!("Current hashes not equal");
                return false;
            }

            if previous_block.hash() != current_block.previous_hash {
                println!("Previous hashes not equal");
                return false;
            }
        }

        return true;
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, block) in self.blocks.iter().enumerate() {
            writeln!(f, "Block {} Hash: {}", index, block.hash());
        }

        Ok(())
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.push(Block::new("I AM THE GENESIS BLOCK, THE ONE AND ONLY.", "0"));

    {
        let second_block = Block::new("Second block mofos", 
        blockchain.last().unwrap().hash());

        blockchain.push(second_block);
    }

    {
        let block = Block::new("Third block up in the house", 
            blockchain.last().unwrap().hash());

        blockchain.push(block);
    }

    println!("{}", blockchain);
    if blockchain.is_chain_valid() {
        println!("Chain is valid");
    } else {
        println!("Uh oh! Chain is invalid");
    }
}
