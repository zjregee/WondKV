use std::fs::File;
use crate::utils::crc32;
use crate::storage::entry;

pub struct DBFile {
    id: u32,
    path: String,
    offset: u32,
    file: File
}

impl DBFile {
    pub fn new(path: String, file_id: u32) -> Option<DBFile> {
        None
    }

    pub fn read(&self, mut offset: u32) -> Option<entry::Entry> {
        let buf = self.read_buf(offset, entry::ENTRY_HEADER_SIZE);
        if buf.is_none() {
            return None;
        }
        let buf = buf.unwrap();
        let mut entry = entry::Entry::decode(buf);
        offset += entry::ENTRY_HEADER_SIZE;
        let key = self.read_buf(offset, entry.meta.key_size);
        if key.is_none() {
            return None;
        }
        entry.meta.key = key.unwrap();
        offset += entry.meta.key_size;
        let value = self.read_buf(offset, entry.meta.value_size);
        if value.is_none() {
            return None;
        }
        entry.meta.value = value.unwrap();
        offset += entry.meta.extra_size;
        let extra = self.read_buf(offset, entry.meta.extra_size);
        if extra.is_none() {
            return None;
        }
        entry.meta.extra = extra.unwrap();
        let crc = crc32::Crc::<u32>::new(&crc32::CRC_32_ISCSI);
        let check_sum = crc.checksum(&entry.meta.value);
        if check_sum != entry.crc32 {
            return None;
        }
        entry.valid = true;
        Some(entry)
    }

    pub fn read_buf(&self, offset: u32, len: u32) -> Option<Vec<u8>> {

        None
    }

    pub fn write(&mut self, entry: entry::Entry) -> bool {
        if !entry.valid {
            return false;
        }
        let buf = entry.encode();
        if buf.is_none() {
            return false;
        }
        let buf = buf.unwrap();

        self.offset += entry.size();
        true
    }

    pub fn close(&mut self) -> bool {

        false
    }

    pub fn sync() -> bool {

        false
    }
}

pub fn build(path: String) {
    
}