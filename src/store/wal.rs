use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::Path,
};

use super::{index::Index, index::LogPointer};

pub struct Wal {
    writer: BufWriter<File>,
    reader: BufReader<File>,
}

impl Wal {
    // Create or Open given file path in append mode
    pub fn open(path: &Path) -> Self {
        std::fs::create_dir_all(path).unwrap();

        let wal_path = path.join("wal.log");

        // Ensure WAL exists WITHOUT truncation
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&wal_path)
            .unwrap();

        let file = OpenOptions::new().append(true).open(&wal_path).unwrap();

        let reader = OpenOptions::new().read(true).open(&wal_path).unwrap();

        Self {
            writer: BufWriter::new(file),
            reader: BufReader::new(reader),
        }
    }

    // OPERATION | key length | value length | key | value
    // PUT      -> 1
    pub fn append_put(&mut self, key: &[u8], value: &[u8]) {
        self.writer.write_all(&[1]).unwrap();
        self.writer
            .write_all(&(key.len() as u32).to_le_bytes())
            .unwrap();
        self.writer
            .write_all(&(value.len() as u32).to_le_bytes())
            .unwrap();

        self.writer.write_all(key).unwrap();
        self.writer.write_all(value).unwrap();
    }

    // OPERATION | key length | key
    // DELETE   -> 2
    pub fn append_del(&mut self, key: &[u8]) {
        self.writer.write_all(&[2]).unwrap();
        self.writer
            .write_all(&(key.len() as u32).to_le_bytes())
            .unwrap();

        self.writer.write_all(key).unwrap();
    }

    // Clears the file writer content
    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }

    pub fn replay(path: &std::path::Path) -> Index {
        let file = File::open(path.join("wal.log")).unwrap();
        let mut reader = BufReader::new(file);

        let mut index = Index::new();
        let mut offset = 0u64;

        loop {
            let mut record_type = [0u8; 1];
            if reader.read_exact(&mut record_type).is_err() {
                break;
            }

            match record_type[0] {
                1 => {
                    // Logic for reading PUT entry
                    let mut buf = [0u8; 4];

                    reader.read_exact(&mut buf).unwrap();
                    let key_len = u32::from_le_bytes(buf);

                    reader.read_exact(&mut buf).unwrap();
                    let val_len = u32::from_le_bytes(buf);

                    let mut key = vec![0; key_len as usize];
                    reader.read_exact(&mut key).unwrap();

                    let value_offset = offset + 1 + 4 + 4 + key_len as u64;

                    reader.seek(SeekFrom::Current(val_len as i64)).unwrap();

                    index.insert(
                        key.clone(),
                        LogPointer {
                            offset: value_offset,
                            value_len: val_len,
                        },
                    );

                    offset += 1 + 4 + 4 + key_len as u64 + val_len as u64;

                    println!(
                        "REPLAY PUT key={:?} value_offset={} value_len={}",
                        key, value_offset, val_len
                    );
                }
                2 => {
                    let mut buf = [0u8; 4];
                    reader.read_exact(&mut buf).unwrap();
                    let key_len = u32::from_le_bytes(buf);

                    let mut key = vec![0; key_len as usize];
                    reader.read_exact(&mut key).unwrap();

                    index.remove(&key);
                    offset += 1 + 4 + key_len as u64;
                }
                _ => panic!("Unknown record type"),
            }
        }
        index
    }

    pub fn read_val(&mut self, offset: u64, len: u32) -> Vec<u8> {
        self.reader.seek(SeekFrom::Start(offset)).unwrap();

        let mut buf = vec![0; len as usize];
        self.reader.read_exact(&mut buf).unwrap();

        buf
    }
}
