//! 本プログラムは標準入力を受け取り
//! 指定されたdelimiterによって分割するし、
//! cliの選択肢を表示するプログラムです

use dialoguer::Select;
use std::io::stdin;

fn main() {
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).unwrap();
    let white_pos: usize = search_whitespace(&input_str);
    println!("{}", white_pos);
    let selection_strs = split_strings(&input_str[white_pos..], &input_str[..white_pos]);
    let index = Select::new()
        .with_prompt("select item")
        .items(&selection_strs)
        .interact()
        .unwrap();
    println!("{}", selection_strs[index]);
}

///
/// 空白文字を検索して、最初にヒットした位置を返します
///
fn search_whitespace(input_str: &str) -> usize {
    for (i, c) in input_str.chars().enumerate() {
        if c == ' ' {
            return i;
        }
    }
    panic!("分割文字列が与えられませんでした");
}

/// 指定された文字列を指定されたdelimiterで分割します
///
///
fn split_strings(input_str: &str, delimiter: &str) -> Vec<String> {
    return input_str
        .trim()
        .split(delimiter)
        .map(|s| s.to_string())
        .collect();
}
