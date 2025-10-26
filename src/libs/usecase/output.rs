const BUFF: usize = 256 * 2;

pub trait Domain {
    fn println(string: [u16; BUFF]) -> Result<(), u32>;
}
