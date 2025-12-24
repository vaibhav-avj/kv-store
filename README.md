# Disk-Backed Key-Value Store (Rust)

A beginner-friendly, **crash-safe, append-only, persistent key-value store** implemented in Rust using a Write-Ahead Log (WAL) and an offset-based in-memory index.

Built to demonstrate:
- Durable **append-only storage**
- **Deterministic crash recovery** via WAL replay
- **Random-access value reads** using byte offsets + seek
- **Atomic log compaction** without data corruption
- Memory-safe design (no unsafe code on storage paths)
- Binary-safe keys and values (`Vec<u8>`)


## Core Features

- `put(key, value)` — insert or overwrite a key
- `get(key)` — read latest value from disk via index lookup + WAL seek
- `delete(key)` — remove a key (tombstoned in WAL, purged on compaction)
- `compact()` — synchronous **blocking log compaction** (rewrites only live keys)
- Append-only log file storage (`wal.log`)
- In-memory index mapping `key → LogPointer(offset, value_len)`
- Full index rebuild on startup using WAL replay


## Architecture

```
Clients
  │  (writes)
  ▼
┌────────────────────────────┐
│  WAL (Append-Only Log)     │  → wal.log
└──────────────┬─────────────┘
               │  (latest offset stored in index)
               ▼  (reads via seek)
┌────────────────────────────┐
│  In-Memory Offset Index    │  → HashMap<Vec<u8>, LogPointer>
└────────────────────────────┘
```


## Storage Format (WAL Records)

```
PUT =  [1:u8][key_len:u32][val_len:u32][key_bytes][val_bytes]
DEL =  [2:u8][key_len:u32][key_bytes]
```

All writes are length-prefixed for deterministic replay and corruption-free reads.


## Usage Example

```rust
use std::path::PathBuf;
use rust_kv_store::KvStore;

fn main() {
    let path = PathBuf::from("./data");
    let mut store = KvStore::open(path);

    store.put(b"a".to_vec(), b"value1".to_vec());
    println!("{:?}", store.get(b"a")); // Some([..])
}
```


## Design Principles

- **WAL is history, index is truth**  
  Values are never duplicated in memory.

- **Offsets are sacred**  
  Index stores exact byte offsets for deterministic reads.

- **Atomic file swap for compaction**  
  `std::fs::rename` ensures full crash safety during log replacement.

- **Scoped locking**  
  Read and write locks are held only for minimal scopes to prevent deadlocks.


## Limitations (Intentionally Out of Scope)

- No generics
- No async runtime
- No LSM or segment-based storage
- No eviction policy beyond deletes + TTL-free compaction


## Next Enhancements

- Checksums for WAL records
- Range scans
- Optional fsync policies
- Segmented WAL compaction
- Async read/write support

