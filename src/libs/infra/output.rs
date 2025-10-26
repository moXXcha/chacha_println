use crate::libs::domain::repository::Infra;
use uefi::proto::console::text::Output;

const BUFF: usize = 256 * 2;

pub struct UEFI;

// Global static for UEFI stdout
static mut STDOUT: Option<*mut Output> = None;

impl UEFI {
    /// Initialize the global stdout pointer
    /// This should be called once at the start of your UEFI application
    /// with the SystemTable from the entry point
    pub unsafe fn init_stdout(stdout: *mut Output) {
        unsafe {
            STDOUT = Some(stdout);
        }
    }
}

impl Infra for UEFI {
    fn output(string: [u16; BUFF]) -> Result<(), u32> {
        unsafe {
            if let Some(stdout) = STDOUT {
                // Find null terminator
                let mut len = 0;
                for (i, &ch) in string.iter().enumerate() {
                    if ch == 0 {
                        len = i;
                        break;
                    }
                }

                // Create null-terminated string for UEFI
                let mut output_buf = [0u16; BUFF];
                output_buf[..len].copy_from_slice(&string[..len]);
                output_buf[len] = 0; // Ensure null termination

                // Call UEFI output_string
                match (*stdout).output_string(&uefi::CStr16::from_u16_with_nul(&output_buf[..=len]).unwrap()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.status().0.try_into().unwrap_or(1)),
                }
            } else {
                // STDOUT not initialized
                Err(1)
            }
        }
    }
}
