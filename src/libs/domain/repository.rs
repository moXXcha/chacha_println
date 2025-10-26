const BUFF: usize = 256 * 2;

pub trait Infra {
    fn output(string: [u16; BUFF]) -> Result<(), u32>;
}
