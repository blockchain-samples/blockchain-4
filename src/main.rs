extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

static DIFFICULTY: usize = 2;

struct Block {
    hash: String,
    pub previous_hash: String,
    pub data: String,
    pub timestamp: u64,
    nonce: i32
}

impl Block {
    pub fn new(data: &str, previous_hash: &str) -> Self {
        
        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

        let timestamp = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;

        let nonce = 0;

        let hash = Self::calculate_hash(previous_hash, timestamp, nonce, data);

        Self {
            data: data.to_string(),
            hash,
            previous_hash: previous_hash.to_string(),
            timestamp,
            nonce
        }
    }

    pub fn calculate_hash(previous_hash: &str, timestamp: u64, nonce: i32, data: &str) -> String {
        let mut sha = Sha256::new();
        let input = format!("{}{}{}{}", previous_hash, timestamp, nonce, data);
        sha.input_str(&input);
        let hash = sha.result_str();

        hash
    }

    pub fn hash(&self) -> String {
        Self::calculate_hash(
            &self.previous_hash, 
            self.timestamp, 
            self.nonce, 
            &self.data
        )
        // &self.hash
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        while &self.hash()[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.hash();
        }

        println!("Block mined: {}", self.hash);
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

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn blocks_mut(&mut self) -> &mut [Block] {
        &mut self.blocks
    }

    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn last(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn is_chain_valid(&self) -> bool {
        let hash_target = "0".repeat(DIFFICULTY);

        for i in 1..self.blocks.len() {
            let current_block = self.blocks.get(i).expect("No blocks found");
            let previous_block = self.blocks.get(i - 1).expect("Need at least 2 blocks");

            // Compare registered hash and calculated hash
            if current_block.hash() != Block::calculate_hash(&current_block.previous_hash, current_block.timestamp, current_block.nonce, &current_block.data) {
                println!("Current hashes not equal");
                return false;
            }

            // Compare previous hash and registered previous hash
            if previous_block.hash() != current_block.previous_hash {
                println!("Previous hashes not equal");
                return false;
            }

            // check if hash is solved
            if &current_block.hash()[..DIFFICULTY] != hash_target {
                println!("Block has not been mined");
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

    {
        blockchain.push(Block::new("I AM THE GENESIS BLOCK, THE ONE AND ONLY.", "0"));
    }

    println!("Trying to mine block 1");
    blockchain.blocks_mut().get_mut(0).unwrap().mine_block(DIFFICULTY);

    {
        let second_block = Block::new(
            "Second block mofos", 
            &blockchain.last().unwrap().hash());

        blockchain.push(second_block);
    }

    println!("Trying to mine block 2");
    blockchain.blocks_mut().get_mut(1).unwrap().mine_block(DIFFICULTY);

    {
        let block = Block::new(
            "Third block up in the house", 
            &blockchain.last().unwrap().hash());

        blockchain.push(block);
    }

    println!("Trying to mine block 3");
    blockchain.blocks_mut().get_mut(2).unwrap().mine_block(DIFFICULTY);

    if blockchain.is_chain_valid() {
        println!("Chain is valid");
    } else {
        println!("Uh oh! Chain is invalid");
    }
}
