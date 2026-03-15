use chrono::DateTime;
use chrono::Utc;
use sha2::{Sha256, Digest};
fn new_transaction(index: u64, data: String, block_hash: String) -> Transaction {
    let timestamp = Utc::now();

    let mut sha256 = Sha256::new();
    let hash_input = format!("{}{}{}{}", index, timestamp, data, block_hash);
    sha256.update(hash_input);
    let hash = format!("{:x}", sha256.finalize());


    Transaction {
        index,
        timestamp,
        data,
        block_hash,
        hash,
    }
}
fn new_block(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Block {
    let timestamp = Utc::now();
    let mut tx_hashes = String::new();
    for tx in &transactions {
        tx_hashes.push_str(&tx.hash);
    }


    let hash_input = format!("{}{}{}{}", index, timestamp, tx_hashes, previous_hash);

    let mut sha256 = Sha256::new();
    sha256.update(hash_input);
    let hash = format!("{:x}", sha256.finalize());


    Block {
        index,
        timestamp,
        transactions,
        previous_hash,
        hash,
    }
}
struct Transaction {
    sender: String,       
    receiver: String,  
    amount: u64,         
    timestamp: DateTime<Utc>,
    hash: String,   
}

struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}

struct Wallet {
    address: String,  
    balance: u64,     
}

fn main() {
    
}