use crate::libs::domain::repository::Infra;
use crate::libs::infra::output;
use crate::libs::usecase::Domain;

const BUFF: usize = 256 * 2;
pub struct Println;

impl Domain for Println {
    fn println(string: [u16; BUFF]) -> Result<(), u32> {
        let result: Result<(), [char; BUFF]>;

        result = output::UEFI::output(string);
        match result {
            Ok(()) => {}
            Err(error_string) => {
                return Err(error_string);
            }
        }
        Ok(())
    }
}
