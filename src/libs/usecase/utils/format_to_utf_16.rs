const BUFF: usize = 256;
const UTF16_BUFF: usize = BUFF * 2;

#[allow(dead_code)]
pub fn format_to_utf_16(string: [char; BUFF]) -> Result<[u16; UTF16_BUFF], ()> {
    let mut utf_16_char: [u16; UTF16_BUFF];
    let mut utf_16_len: usize;

    utf_16_char = [0u16; UTF16_BUFF];
    utf_16_len = 0;
    for charactar in string {
        let mut buff: [u16; 2];
        let encoded: &[u16];

        buff = [0u16; 2];
        encoded = charactar.encode_utf16(&mut buff);
        for &u in encoded {
            utf_16_char[utf_16_len] = u;
            utf_16_len += 1;
        }
    }
    Ok(utf_16_char)
}
