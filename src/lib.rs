#[macro_export]
macro_rules! chacha_println {
    () => {
      {
         $crate::print_test(&['\n']);
      }
    };
    ($($args:tt)*) => {
      {
        let s = format_chacha_println!($($args)*);
        $crate::print_test(&s);
      }
    };
}

#[allow(dead_code)]
pub fn print_test(chars: &[char]) {
    println!("{:?}", chars[0]);
}
