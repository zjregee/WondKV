use crate::{wondkv::WondKV, storage};

pub fn default_config() -> Config {
    Config {
        dir_path: "/tmp".to_string(),
    }
}

pub struct Config {
    dir_path: String
}

impl Config {
    pub fn open(&self) -> Option<WondKV> {
        let res = storage::db_file::build(self.dir_path.clone());
        if res.is_none() {
            return None;
        }

        
        todo!()
    }
}