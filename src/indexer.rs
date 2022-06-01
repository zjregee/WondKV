use crate::storage::entry;

pub struct Indexer {
    pub file_id: u32,
    pub offset: u32, 
    pub meta: entry::Meta,
}