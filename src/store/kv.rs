// Invariant:
// 1. All writes are append-only to disk (WAL)
// 2. In-memory index is the source of truth for reads
// 3. On startup, index is rebuilt from WAL

use std::path::PathBuf;

pub struct KvStore {
    
}

impl KvStore {
    pub fn open(path: PathBuf) -> Self {
        todo!()
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        todo!()
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        todo!()
    }

    pub fn delete(&self, key: &[u8]) {
        todo!()
    }
}