#![allow(unused_imports, dead_code)]

use skiplist::SkipMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};


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



}