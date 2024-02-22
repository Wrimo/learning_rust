// 2/19/2024
// Rust implementation of byte pair encoding
// https://en.wikipedia.org/wiki/Byte_pair_encoding

use std::collections::HashMap;
use std::io;

fn main() {
    let mut input: String = String::new();
    let _ = io::stdin().read_line(&mut input);

    let mut text = input;
    let mut loop_count: u8 = 0;
    let mut conversions: Vec<String> = Vec::new();
    loop {
        let mut counts: HashMap<String, i32> = HashMap::new();
        let chars: Vec<char> = text.chars().collect();

        for i in 0..chars.len() - 1 {
            let s: String = format!("{}{}", chars[i], chars[i + 1]);

            match counts.get(&s) {
                Some(count) => {
                    counts.insert(s, count + 1);
                }
                None => {
                    counts.insert(s, 1);
                }
            }
        }

        let max_sequence: Option<(&String, &i32)> = counts
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, v)| (k, v));
        match max_sequence {
            Some(x) => {
                if *x.1 < 2 {
                    break;
                }
                conversions.push(x.0.to_string());
                let symbol: String = format!("{}", (90 - loop_count) as char);
                text = text.replace(x.0, &symbol);
            }
            None => {
                break;
            }
        }
        loop_count += 1;
    }

    println!("{}", text);
    for i in (0..conversions.len()).rev(){
        let symbol: String = format!("{}", (90 - u8::try_from(i).unwrap()) as char);
        println!("{}={}", symbol, conversions[i]);
    }
}
