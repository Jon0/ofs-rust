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


pub struct PartialMessage {

}


impl PartialMessage {
    pub fn new() {

    }


    pub fn read_some<T: Readable>(stream: &T) {


    }
}
