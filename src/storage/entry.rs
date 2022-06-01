pub const ENTRY_HEADER_SIZE: u32 = 26;

pub struct Meta {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub extra: Vec<u8>,
    pub key_size: u32,
    pub value_size: u32,
    pub extra_size: u32
}

pub struct Entry {
    pub meta: Meta,
    pub state: u16,
    pub crc32: u32,
    pub time_stamp: u64,
}

impl Entry {
    pub fn new(key: Vec<u8>, value: Vec<u8>, extra: Vec<u8>, t: u16, mark: u16) -> Entry {
        let mut state = 0;
        state = state | (t << 8);
        state = state | mark;
        todo!()
    }

    pub fn new_no_extra() -> Entry {
        todo!()
    }

    pub fn new_with_expire() -> Entry {
        todo!()
    }

    pub fn size(&self) -> u32 {
        ENTRY_HEADER_SIZE + self.meta.key_size + self.meta.value_size + self.meta.extra_size
    }

    pub fn encode(&self) -> Option<Vec<u8>> {
        None
    }

    pub fn decode(&mut self, buf: Vec<u8>) -> bool {
        true
    }

    pub fn get_type(&self) -> u16 {
        self.state >> 8
    }

    pub fn get_mark(&self) -> u16 {
        self.state & (2<<7 - 1)
    }
}