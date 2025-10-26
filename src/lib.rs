pub mod libs;

#[allow(dead_code)]
pub const BUFF: usize = 256;
#[allow(dead_code)]
pub const ROW: usize = 256;

#[macro_export]
macro_rules! chacha_println {
    () => {
      {
        $crate::print_test(['\0'; $crate::BUFF]);
      }
    };
    ($($args:expr)*) => {
      {
        let strings: &[&str];
        let buff: u32;
        let formated_strings_result: [[char; BUFF]; ROW];
        let formated_string: [char; BUFF];
        let formated_utf_16: [u16; UTF16_BUFF];

        strings = &[$($args),*];
        formated_strings = format_string_to_char_arraies(strings)?;
        formated_string = format(formated_strings)?;
        formated_utf_16 = format_to_utf_16(formated_string)?;
        status = domain::Prinltn::println(formated_utf_16);
        if status != 0 {

        }
      }
    };
}

#[allow(dead_code)]
pub fn print_test(chars: [char; BUFF]) {
    println!("{:?}", chars[0]);
}
