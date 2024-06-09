use skiplist::SkipMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemTableEntry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: u128,
    pub deleted: bool,
}

pub struct MemTable {
    entries: SkipMap<Vec<u8>, MemTableEntry>,
    size: usize,
    max_size: usize,
}

impl MemTable {
    pub fn new(max_size: usize) -> MemTable {
        MemTable {
            entries: SkipMap::new(),
            size: 0,
            max_size,
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
        self.check_flush();
    }

    pub fn get(&self, key: &[u8]) -> Option<MemTableEntry> {
        self.entries.get(key).map(|entry| entry.clone())
    }

    pub fn delete(&mut self, key: &[u8]) {
        let timestamp = Self::current_timestamp();
        let entry = MemTableEntry {
            key: key.to_vec(),
            value: Vec::new(),
            timestamp,
            deleted: true,
        };
        self.size += key.len() + 16 + 1;
        self.entries.insert(key.to_vec(), entry);
        self.check_flush();
    }

    // pub fn range_scan(&self, start: &[u8], end: &[u8]) -> Vec<(Vec<u8>, MemTableEntry)> {
    //     self.entries.range(start.to_vec()..end.to_vec())
    //         .map(|entry| (entry.key.clone(), entry.value.clone()))
    //         .collect()
    // }

    fn check_flush(&self) {
        if self.size >= self.max_size {
            println!("Flushing MemTable to SSTable...");
            // Implement actual flush logic here
        }
    }

    fn current_timestamp() -> u128 {
        let start = SystemTime::now();
        start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
    }
}
