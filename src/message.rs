use std::mem;

pub trait InputStream {
    
}


pub trait Readable {
    fn read(&self, buf: &mut [u8]) -> Result<usize, i32>;
}


pub struct MsgHeader {
    data_size:  u32,
    data_hash:  u32,
    data_type:  u32,
    event_type: u32,
    event_id:   u32,
}


impl MsgHeader {
    pub fn read_some<T: Readable>(&mut self, stream: &T) {

    }
}


pub struct PartialMessage {
    head: MsgHeader,
}


impl PartialMessage {
    pub fn new() {

    }


    pub fn read_some<T: Readable>(&mut self, stream: &T) {
        unsafe {
            let mut raw_bytes : &[u8; 8] = mem::transmute(&mut self.head);
        }
    }
}
