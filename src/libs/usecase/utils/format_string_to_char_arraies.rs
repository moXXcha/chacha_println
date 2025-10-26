const BUFF: usize = 256;
const ROW: usize = 256;

pub fn format_string_to_char_arraies(strings: &[&str]) -> Result<[[char; BUFF]; ROW], ()> {
    let mut ans_strings: [[char; BUFF]; ROW];
    let mut str_count: usize;

    ans_strings = [['\0'; BUFF]; ROW];
    str_count = 0;
    for string in strings {
        let mut chars: [char; BUFF];
        let mut len: usize;

        chars = ['\0'; BUFF];
        len = 0;
        for character in string.chars() {
            if len >= BUFF {
                break;
            }
            chars[len] = character;
            len += 1;
        }
        ans_strings[str_count] = chars;
        str_count += 1;
    }
    Ok(ans_strings)
}
