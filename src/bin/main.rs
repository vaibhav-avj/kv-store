use kv_store::KvStore;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./data");

    let mut store = KvStore::open(path);
    store.put(b"a".to_vec(), b"1".to_vec());
    store.put(b"a".to_vec(), b"2".to_vec());
    store.put(b"b".to_vec(), b"9".to_vec());

    store.compact();

    println!("Compact hogyaa bhai");

    assert_eq!(store.get(b"a"), Some(b"2".to_vec()));
    assert_eq!(store.get(b"b"), Some(b"9".to_vec()));
}
