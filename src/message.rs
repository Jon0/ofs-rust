pub trait Readable {
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, i32>;
}



pub struct PartialMessage {

}


impl PartialMessage {
    pub fn new() {

    }
}
