use std::path::PathBuf;
use kv_store::KvStore;

fn main() {
    let path = PathBuf::from("./data");
    let mut store = KvStore::open(path);

    store.put(b"key1".to_vec(), b"value1".to_vec());
    assert_eq!(store.get(b"key1"), Some(b"value1".to_vec()));

    store.delete(b"key1");
    assert_eq!(store.get(b"key1"), None);
}
