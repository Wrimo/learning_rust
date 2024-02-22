use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin(); 
    let mut message: String = stdin.lock().lines().next().unwrap().unwrap(); 
    for line in stdin.lock().lines() {
        let line: String = line.unwrap(); 
        
        let symbol: String = line.chars().take(1).collect(); 
        let conversion: String = line.chars().skip(2).take(2).collect();

        message = message.replace(&symbol, &conversion);
    }
    println!("{}", message);
}