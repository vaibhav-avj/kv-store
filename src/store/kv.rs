// Invariant:
// 1. All writes are append-only to disk (WAL)
// 2. In-memory index is the source of truth for reads
// 3. On startup, index is rebuilt from WAL

// Compaction invariant:
// - No writes during compaction
// - Old WAL remains untouched until new WAL is fully written
// - WAL swap is atomic
// - Index is rebuilt from compacted WAL

use std::fs::rename;
use std::{fs::OpenOptions, io::Write, path::PathBuf};

use super::index::Index;
use super::wal::Wal;

pub struct KvStore {
    wal: Option<Wal>,
    index: Index,
    path: PathBuf,
}

impl KvStore {
    pub fn open(path: PathBuf) -> Self {
        let wal = Some(Wal::open(&path));
        let index = Wal::replay(&path);

        Self { wal, index, path }
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.wal.as_mut().unwrap().append_put(&key, &value);
        self.wal.as_mut().unwrap().flush();
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        let ptr = self.index.get(key)?;
        Some(
            self.wal
                .as_mut()
                .unwrap()
                .read_val(ptr.offset, ptr.value_len),
        )
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.wal.as_mut().unwrap().append_del(key);
        self.wal.as_mut().unwrap().flush();

        self.index.remove(key);
    }

    pub fn compact(&mut self) {
        println!("inside compact() call");
        let compact_path = self.path.join("wal.compact");

        let mut compact_wal = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&compact_path)
            .unwrap();

        for (key, ptr) in self.index.iter() {
            let value = self
                .wal
                .as_mut()
                .unwrap()
                .read_val(ptr.offset, ptr.value_len);

            compact_wal.write_all(&[1]).unwrap();
            compact_wal
                .write_all(&(key.len() as u32).to_le_bytes())
                .unwrap();
            compact_wal
                .write_all(&(value.len() as u32).to_le_bytes())
                .unwrap();
            compact_wal.write_all(key).unwrap();
            compact_wal.write_all(&value).unwrap();
        }

        compact_wal.flush().unwrap();

        let wal_path = self.path.join("wal.log");
        let old = self.wal.take();
        drop(old);

        rename(&compact_path, &wal_path).unwrap();

        self.index = Wal::replay(&self.path);
        self.wal = Some(Wal::open(&self.path));
    }
}
