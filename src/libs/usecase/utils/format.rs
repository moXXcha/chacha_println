const BUFF: usize = 256;
const ROW: usize = 256;

pub fn format(strings: [[char; BUFF]; ROW]) -> Result<[char; BUFF], u32> {
    let mut ans_string: [char; BUFF];
    let mut ans_string_len: usize;
    let mut arg_string_count: usize;
    let mut braces_pair_count: usize;
    let mut brace_count: usize;
    let mut prev_char: char;
    let mut char_count: usize;

    ans_string = ['\0'; BUFF];
    ans_string_len = 0;
    arg_string_count = 0;
    braces_pair_count = 0;
    brace_count = 0;
    prev_char = '\0';
    char_count = 0;
    for charactar in strings[0] {
        if charactar == '\0' {
            break;
        }
        if ans_string_len >= BUFF {
            return Err(1);
        }
        if prev_char == '{' && charactar == '}' {
            if braces_pair_count > (strings.len() - 1) {
                return Err(1);
            }
            if brace_count % 2 != 0 {
                if !is_close_brace_count_multipul_two(strings[0], &char_count, brace_count) {
                    return Err(1);
                }
                // 対象の数だけbraceを入れる
                append_braces(brace_count, &mut ans_string, &mut ans_string_len);
                // 入力された文字を入れる
                for charactar in strings[arg_string_count + 1] {
                    if charactar == '\0' {
                        break;
                    }
                    if ans_string_len >= BUFF {
                        break;
                    }
                    ans_string[ans_string_len] = charactar;
                    ans_string_len += 1;
                }
                braces_pair_count += 1;
                arg_string_count += 1;
            }
            brace_count = 1;
            char_count += 1;
            prev_char = charactar;
            continue;
        }
        if (charactar == '{' || charactar == '}') && (prev_char == charactar) {
            brace_count += 1;
            char_count += 1;
            prev_char = charactar;
            continue;
        }
        if charactar == '{' || charactar == '}' {
            brace_count = 1;
            char_count += 1;
            prev_char = charactar;
            continue;
        }
        if prev_char == '{' || prev_char == '}' {
            if brace_count % 2 != 0 {
                return Err(1);
            }
            // TODO: ans_stringにbraceを入れる
            append_braces(brace_count, &mut ans_string, &mut ans_string_len);
            char_count += 1;
            prev_char = charactar;
            continue;
        }
        ans_string[ans_string_len] = charactar;
        ans_string_len += 1;
        char_count += 1;
        prev_char = charactar;
    }
    Ok(ans_string)
}

fn is_close_brace_count_multipul_two(
    string: [char; BUFF],
    current_string_count: &usize,
    brase_count: usize,
) -> bool {
    let mut close_brace_count: usize;
    let mut string_count: usize;
    let string_len: usize;

    close_brace_count = 0;
    string_count = *current_string_count;
    string_len = string.len();
    while string_count < string_len && string[string_count] == '}' {
        close_brace_count += 1;
        string_count += 1;
    }
    if brase_count == close_brace_count {
        return true;
    }
    false
}

fn append_braces(brace_count: usize, ans_string: &mut [char; BUFF], ans_string_len: &mut usize) {
    let mut loop_count: usize;

    loop_count = 0;
    // 対象の数だけbraceを入れる
    loop {
        if loop_count == (brace_count - 1) / 2 {
            break;
        }
        if *ans_string_len >= BUFF {
            break;
        }
        ans_string[*ans_string_len] = '{';
        *ans_string_len += 1;
        loop_count += 1;
    }
}
