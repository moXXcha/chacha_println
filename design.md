# println!の再現
println!の再現するぞー！！！
# 概要
勉強のためにprintln!の再現をしたい
# 条件
no_std
最終的なprint動作は「UEFI」の提供の関数で行う
# 必要な要素
- 呼び出し元のmain関数
- println!最上位println!マクロ（引数が可変長のためマクロ）
- "{}"を受け取った引数に置き換えるformatのマクロ（引数が可変長のためマクロ)
- UEFI提供関数を呼び出すOutput
# 作成するファイル（せっかくだしクリーンアーキテクチャっぽく）
- main.rs
- lib.rs
- lib/
  - usecase/
    - utils
      - find_curly_braces_pairs.rs
      - format_chacha_println.rs
    - output.rs //traitの定義
  - domain/
    - output.rs //traitの実装
  - infra/
    - UEFI_output.rs

# 作成する関数
- src/main@main()
- src/lib.rs@main()
- src/lib.rs@chacha_println!($(args::tt)*)
- src/lib/usecase/utils/format_chacha_println.rs@format_chacha_println!($($args:tt)*)
- src/lib/infra/UEFI_output.rs@output(chars: &[char]) -> Result<(), ()>
- src/lib/infra/output_UEFI.rs@encode_utf16(chars: &[char], target: &mut [u16] ) ->Result<usize, ()>
- src/lib/usecase/utils/find_curly_braces_pairs.rs@find_curly_braces_pairs(current: &char, prev: Option<&char>, next: Option<&char>) -> bool
- src/lib/domain/output.rs@output(chars: &[char], buf_size: usize) -> Result<(), ()>
- src/lib/usecase/output.rs@output (trait)
