#![no_std]

extern crate alloc;

use uefi::allocator::Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub mod libs;

pub const BUFF: usize = 256;
pub const ROW: usize = 256;
pub const UTF16_BUFF: usize = BUFF * 2;

#[macro_export]
macro_rules! chacha_println {
    () => {
      {
        // Empty println - do nothing
      }
    };
    ($($args:expr),*) => {
      (|| {
        use $crate::libs::usecase::Domain;

        let strings: &[&str];
        let formated_strings: [[char; $crate::BUFF]; $crate::ROW];
        let formated_string: [char; $crate::BUFF];
        let formated_utf_16: [u16; $crate::UTF16_BUFF];

        strings = &[$($args),*];
        match $crate::libs::usecase::utils::format_string_to_char_arraies::format_string_to_char_arraies(strings) {
            Ok(fs) => formated_strings = fs,
            Err(_) => return,
        }
        match $crate::libs::usecase::utils::format::format(formated_strings) {
            Ok(fs) => formated_string = fs,
            Err(_) => return,
        }
        match $crate::libs::usecase::utils::format_to_utf_16::format_to_utf_16(formated_string) {
            Ok(fu) => formated_utf_16 = fu,
            Err(_) => return,
        }
        let _ = $crate::libs::domain::service::Println::println(formated_utf_16);
      })()
    };
}

// print_test is not available in no_std environment
// pub fn print_test(chars: [char; BUFF]) {
//     println!("{:?}", chars[0]);
// }
