use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LogPointer {
    pub offset: u64,
    pub value_len: u32,
}

pub type Index = HashMap<Vec<u8>, LogPointer>;