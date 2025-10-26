use crate::libs::domain::repository::Infra;
use uefi::proto::console::text::Output;

const BUFF: usize = 256 * 2;
pub struct UEFI {
    con_out: *mut SimpleTextOutputProtocol,
}

impl Infra for UEFI {
    fn output(string: [u16; BUFF]) -> Result<(), u32> {
        let mut buff: [u16; BUFF];
        let copy_len: Option<unknow>;
        let status: unknown;

        buff = [0u16; BUFF];
        copy_len = len.min(511);
        buf[..copy_len].copy_from_slice(&string[..copy_len]);
        buf[copy_len] = 0;
        status = unsafe { ((*self.con_out).output_string)(self.con_out, buf.as_ptr()) };
        if status != 0 {
            Err(status)
        }
        Ok(())
    }
}
