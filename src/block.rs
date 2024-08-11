#![allow(unused)]

use serde::{Serialize, Deserialize};
use std::collections::LinkedList as List;
use sha2::{Sha256, Digest};
use ripemd160::{Ripemd160};
use base58::{ToBase58};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlockChain {
    blocks: List<Block>
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: List::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push_back(block);
    }

    pub fn get_blockchain_info(&self) -> String {
        format!(
            "Height: {}, Latest Block Hash: {:?}",
            self.height(),
            self.latest_block().map(|block| &block.hash)
        )
    }

    pub fn get_block_by_height(&self, height: u64) -> Option<&Block> {
        self.blocks.iter().find(|block| block.height == height)
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.blocks.iter().find(|block| block.hash == hash)
    }
    
    pub fn latest_block(&self) -> Option<&Block> {
        self.blocks.iter().last()
    }

    pub fn height(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn list_blocks(&self) -> Vec<&Block> {
        self.blocks.iter().collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    id: u128,
    hash: String,
    height: u64,
    prev_hash: Option<String>,
    transactions: List<Transaction>,
    timestamp: u64,
}

impl Block {
    pub fn new(id: u128, hash: String, height: u64, prev_hash: Option<String>, transactions: List<Transaction>, timestamp: u64) -> Self {
        Block {
            id,
            hash,
            height,
            prev_hash,
            transactions,
            timestamp,
        }
    }
    
    pub fn calculate_merkle_root(&self) -> String {
        let mut tx_hashes: Vec<String> = self.transactions.iter().map(|tx| tx.txid.clone()).collect();
        while tx_hashes.len() > 1 {
            if tx_hashes.len() % 2 != 0 {
                tx_hashes.push(tx_hashes[tx_hashes.len() - 1].clone());
            }
            let mut new_hashes = Vec::new();
            for i in (0..tx_hashes.len()).step_by(2) {
                let combined = format!("{}{}", tx_hashes[i], tx_hashes[i+1]);
                let mut hasher = Sha256::new();                
                hasher.update(combined.as_bytes());                
                let hash = hasher.finalize();    
                new_hashes.push(format!("{:x}", hash));
            }
            tx_hashes = new_hashes;
        }
        tx_hashes[0].clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: String,
}

impl Transaction {
    pub fn new(inputs: List<TxIn>, outputs: List<TxOut>, txid: String) -> Self {
        Transaction {
            inputs,
            outputs,
            txid,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

struct TxIn {
    prev_txid: String,
    out: usize,
    signature: String, // to spend the output
}

impl TxIn {
    pub fn new(prev_txid: String, out: usize, signature: String) -> Self {
        TxIn {
            prev_txid,
            out,
            signature,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TxOut {
    public_address: String,
    satoshis: u64, 
    // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}

impl TxOut {
    pub fn new(public_address: String, satoshis: u64) -> Self {
        TxOut {
            public_address,
            satoshis,
        }
    }

    pub fn compute_address(public_key: &[u8]) -> String {
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(public_key);
        let sha256_hash = sha256_hasher.finalize();
        let mut ripemd160_hasher = Ripemd160::new();
        ripemd160_hasher.update(&sha256_hash);
        let ripemd160_hash = ripemd160_hasher.finalize();
        let mut extended_ripemd160 = vec![0x00];
        extended_ripemd160.extend_from_slice(&ripemd160_hash.as_ref());
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&extended_ripemd160);
        let first_sha256_hash = sha256_hasher.finalize();
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&first_sha256_hash);
        let second_sha256_hash = sha256_hasher.finalize();    
        let checksum = &second_sha256_hash[0..4];    
        extended_ripemd160.extend_from_slice(checksum);    
        extended_ripemd160.to_base58()
    }
}

// Try to include bitcoin related functionalities like serialization, computing addresses etc.,
// You can add your own methods for different types and associated unit tests


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_block() {
        let mut blockchain = BlockChain::new();
        
        let txin = TxIn::new(String::from("prev_txid_1"), 0, String::from("signature_1"));
        let txout = TxOut::new(String::from("address_1"), 5000000000);
        let mut inputs = List::new();
        inputs.push_back(txin);
        let mut outputs = List::new();
        outputs.push_back(txout);
        let transaction = Transaction::new(inputs, outputs, String::from("txid_1"));
        
        let mut transactions = List::new();
        transactions.push_back(transaction);    

        let block = Block::new(
            1,
            String::from("hash_1"),
            1,
            None,
            transactions,
            1620000000,
        );
        
        blockchain.add_block(block.clone());
        assert_eq!(blockchain.height(), 1);
        assert_eq!(blockchain.latest_block().unwrap().hash, "hash_1");
    }

    #[test]
    fn test_blockchain_serialization() {
        let mut blockchain = BlockChain::new();
        
        let txin = TxIn::new(String::from("prev_txid_1"), 0, String::from("signature_1"));
        let txout = TxOut::new(String::from("address_1"), 5000000000);
        let mut inputs = List::new();
        inputs.push_back(txin);
        let mut outputs = List::new();
        outputs.push_back(txout);
        let transaction = Transaction::new(inputs, outputs, String::from("txid_1"));
        
        let mut transactions = List::new();
        transactions.push_back(transaction);
        
        let block = Block::new(
            1,
            String::from("hash_1"),
            1,
            None,
            transactions,
            1620000000,
        );
        
        blockchain.add_block(block.clone());
        
        let serialized_blockchain = serde_json::to_string(&blockchain).unwrap();
        let deserialized_blockchain: BlockChain = serde_json::from_str(&serialized_blockchain).unwrap();
        
        assert_eq!(deserialized_blockchain.height(), 1);
        assert_eq!(deserialized_blockchain.latest_block().unwrap().hash, "hash_1");
    }

    #[test]
    fn test_calculate_merkle_root() {
        let txin = TxIn::new(String::from("prev_txid_1"), 0, String::from("signature_1"));
        let txout = TxOut::new(String::from("address_1"), 5000000000);
        let mut inputs = List::new();
        inputs.push_back(txin);
        let mut outputs = List::new();
        outputs.push_back(txout);
        let transaction = Transaction::new(inputs.clone(), outputs.clone(), String::from("txid_1"));
        let transaction2 = Transaction::new(inputs, outputs, String::from("txid_2"));
        
        let mut transactions = List::new();
        transactions.push_back(transaction);
        transactions.push_back(transaction2);
        
        let block = Block::new(1, String::from("hash_1"), 1, None, transactions, 1620000000);
        
        let merkle_root = block.calculate_merkle_root();
        println!("Merkle Root: {}", merkle_root);
        assert!(!merkle_root.is_empty());
    }

    #[test]
    fn test_compute_address() {
        let public_key = b"test_public_key";
        let address = TxOut::compute_address(public_key);
        println!("Computed Address: {}", address);
        assert_eq!(address.len(), 34); 
    }

    fn create_sample_block(height: u64, hash: &str, prev_hash: Option<String>) -> Block {
        let txin = TxIn::new(String::from("prev_txid"), 0, String::from("signature"));
        let txout = TxOut::new(String::from("address"), 5000);
        let mut inputs = List::new();
        inputs.push_back(txin);
        let mut outputs = List::new();
        outputs.push_back(txout);
        let transaction = Transaction::new(inputs, outputs, String::from("txid"));
        let mut transactions = List::new();
        transactions.push_back(transaction);

        Block::new(height.into(), hash.to_string(), height, prev_hash, transactions, 1620000000)
    }

    #[test]
    fn test_blockchain_creation() {
        let blockchain = BlockChain::new();
        assert_eq!(blockchain.height(), 0);
        assert!(blockchain.latest_block().is_none());
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = BlockChain::new();
        let block = create_sample_block(1, "hash_1", None);
        blockchain.add_block(block);

        assert_eq!(blockchain.height(), 1);
        assert_eq!(blockchain.latest_block().unwrap().hash, "hash_1");
    }

    #[test]
    fn test_get_block_by_height() {
        let mut blockchain = BlockChain::new();
        let block = create_sample_block(1, "hash_1", None);
        blockchain.add_block(block);

        let retrieved_block = blockchain.get_block_by_height(1);
        assert!(retrieved_block.is_some());
        assert_eq!(retrieved_block.unwrap().hash, "hash_1");
    }

    #[test]
    fn test_get_block_by_hash() {
        let mut blockchain = BlockChain::new();
        let block = create_sample_block(1, "hash_1", None);
        blockchain.add_block(block);

        let retrieved_block = blockchain.get_block_by_hash("hash_1");
        assert!(retrieved_block.is_some());
        assert_eq!(retrieved_block.unwrap().height, 1);
    }

    #[test]
    fn test_get_blockchain_info() {
        let mut blockchain = BlockChain::new();
        let block = create_sample_block(1, "hash_1", None);
        blockchain.add_block(block);

        let info = blockchain.get_blockchain_info();
        assert_eq!(info, "Height: 1, Latest Block Hash: Some(\"hash_1\")");
    }

    #[test]
    fn test_list_blocks() {
        let mut blockchain = BlockChain::new();
        let block1 = create_sample_block(1, "hash_1", None);
        let block2 = create_sample_block(2, "hash_2", Some("hash_1".to_string()));
        blockchain.add_block(block1);
        blockchain.add_block(block2);

        let blocks = blockchain.list_blocks();
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].hash, "hash_1");
        assert_eq!(blocks[1].hash, "hash_2");
    }
}