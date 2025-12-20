use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
};

pub struct Wal {
    writer: BufWriter<File>,
}

impl Wal {
    // Create or Open given file path in append mode
    pub fn open(path: &Path) -> Self {
        std::fs::create_dir_all(path).unwrap();

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.join("wal.log"))
            .unwrap();

        Self {
            writer: BufWriter::new(file),
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
}
