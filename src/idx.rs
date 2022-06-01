use crate::wondkv::WondKV;

pub enum DataType {
    String,
    List,
    Hash,
    Set,
    ZSet,
}

pub enum HashOperation {
    HashHSet,
    HashHDel,
    HashHClear,
    HashHExpire,
}

impl WondKV {
    pub fn build_hash_index() {

    }

    pub fn load_idx_from_files() {
        
    }
}

