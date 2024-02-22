use std::io::{self, BufRead};
mod types;

fn productions_from_str(val: &str) -> (String, String) {
    if val.len() > 1 {
        let chars: Vec<char> = val.chars().collect();
        return (chars[0].to_string(), chars[1].to_string());
    }

    return (val.to_string(), "".to_string());
}

fn get_productions(productions: &mut Vec<types::Production>) {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line: String = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();

        assert!(words.len() >= 3);

        let symbol: String = words[0].to_string();
        let (next_production, concatted_production) = productions_from_str(words[2]);
        let mut next_production1: String = "".to_string();
        let mut concatted_production1: String = "".to_string();

        if words.len() == 5 {
            (next_production1, concatted_production1) = productions_from_str(words[4])
        }

        let prod = types::Production {
            symbol: symbol,
            next_production: next_production,
            concatted_production: concatted_production,
            next_production1: next_production1,
            concatted_production1: concatted_production1,
        };

        productions.push(prod);
    }
    for x in productions {
        println!(
            "{} -> {} {} | {} {}",
            x.symbol,
            x.next_production,
            x.concatted_production,
            x.next_production1,
            x.concatted_production1
        );
    }
}

fn main() {
    let mut productions: Vec<types::Production> = Vec::new();
    get_productions(&mut productions);

    let input: &str = "aaabbbcc";
    let chars = input.chars().collect();

    let mut p: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; productions.len()]; input.len()]; input.len()];
    let mut back: Vec<Vec<Vec<(u32, u32, u32)>>> =
        vec![vec![vec![(0, 0, 0); productions.len()]; input.len()]; input.len()];


    for i in 0..input.len() {
        for j in 0..productions.len() {
            if productions[i]::goesToSymbol(chars[i]) { 
                p[o, i, j] == true; 
            }
        } 
    }

    for l in 1..input.len() { 
        for s in 0..n-l+1 {
            for p in 0..l-1 { 
                for r in 0..productions.len() { 
                    let prod = productions[r]; 
                    
                    // need to get indexes of next symbols productions 
                    //

                    if p[p, s, ]
                }
            }
        }
    }
}
