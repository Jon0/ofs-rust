use std::mem;


pub struct PartialTransfer {
    transferred: u32,
}



pub trait Serializable {

}


fn read_serial<T: Serializable, I: InputStream>(obj: &mut T, input: &mut I, p: &mut PartialTransfer, depth: usize) {

}


pub trait InputStream {
    fn read<T>(&mut self, buf: &mut T) -> Result<usize, i32>;
}


pub trait OutputStream {
    fn write<T>(&mut self, buf: &T) -> Result<usize, i32>;
}


pub struct MsgHeader {
    data_size:  u32,
    data_hash:  u32,
    data_type:  u32,
    event_type: u32,
    event_id:   u32,
}


impl MsgHeader {
    pub fn read_some<T: InputStream>(&mut self, stream: &mut T) {
        stream.read(&mut self.data_size);
        stream.read(&mut self.data_hash);
        stream.read(&mut self.data_type);
        stream.read(&mut self.event_type);
        stream.read(&mut self.event_id);
    }


    pub fn write_some<T: OutputStream>(&self, stream: &mut T) {
        stream.write(&self.data_size);
        stream.write(&self.data_hash);
        stream.write(&self.data_type);
        stream.write(&self.event_type);
        stream.write(&self.event_id);
    }


    pub fn as_bytes(&self) -> &[u8; 8] {
        unsafe {
            let mut raw_bytes : &[u8; 8] = mem::transmute(&self);
            return raw_bytes;
        }
    }
}


pub struct PartialMessage {
    head: MsgHeader,
}


impl PartialMessage {
    pub fn new() {

    }


    pub fn read_some<T: InputStream>(&mut self, stream: &mut T) {
        self.head.read_some(stream);
    }
}


pub struct Server {

}


impl Server {
    pub fn init() -> Server {
        return Server{};
    }
}


pub trait Message {
    fn apply(server: Server);
}
