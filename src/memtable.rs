#![allow(unused_imports, dead_code)]

use skiplist::SkipMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct MemTableEntry {
    pub key:       Vec<u8>,
    pub value:     Vec<u8>,
    pub timestamp: u128,
    pub deleted:   bool
}

pub struct MemTable {
    entries:  SkipMap<Vec<u8>, MemTableEntry>,
    size:     usize,
    max_size: usize
}

impl MemTable {

    pub fn new(max_size: usize) -> MemTable {
        MemTable {
            entries: SkipMap::new(),
            size: 0,
            max_size
        }
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let timestamp = Self::current_timestamp();
        let entry = MemTableEntry {
            key: key.clone(),
            value: value.clone(),
            timestamp,
            deleted: false,
        };
        self.size += key.len() + value.len() + 16 + 1;
        self.entries.insert(key, entry);
        // println!("Entries {:?}", self.entries);
        self.check_flush();
    }

    pub fn get(&self, key: &[u8]) -> Option<MemTableEntry> {
        let v = self.entries.get(key).map(|entry| entry.clone());
        // println!("Value: {:?}", v);
        // println!("Entries {:?}", self.entries);
        v
    }

    pub fn delete(&mut self, key: &[u8]) {
        let timestamp = Self::current_timestamp();
        let entry = MemTableEntry {
            key: key.to_vec(),
            value: Vec::new(),
            timestamp,
            deleted: true,
        };
        self.size += key.len() + 16 + 1; // Update size to include the length of the key, timestamp (16 bytes), and deleted flag (1 byte)
        self.entries.insert(key.to_vec(), entry);
        self.check_flush();
    }

    pub fn check_flush(&self) {
        if self.size > self.max_size {
            println!("Flushing memTable to SSTable....")
            // Yet to be implemented
        }
    }

    pub fn current_timestamp() -> u128 {
        let now = SystemTime::now();
        now.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
    }



}