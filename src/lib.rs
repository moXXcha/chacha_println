pub mod libs;

const BUFF: usize = 256;

#[macro_export]
macro_rules! chacha_println {
    () => {
      {
         $crate::print_test(&['\n']);
      }
    };
    ($($args:expr)*) => {
      {
        let strings: &[&str];
        let buff: u32;
        let formated_strings: [char; BUFF];

        strings = &[$($args),*];
        formated_strings = format_string(strings);
        $crate::print_test(strings);
      }
    };
}

#[allow(dead_code)]
pub fn print_test(chars: &[char]) {
    println!("{:?}", chars[0]);
}

#[allow(dead_code)]
fn format_string(strings: &[&str]) -> [char; BUFF] {
    let mut chars: [char; BUFF];
    let mut len: usize;

    chars = ['\0'; BUFF];
    len = 0;
    for string in strings {
        for character in string.chars() {
            chars[len] = character;
            len += 1;
        }
    }
    return chars;
}
