// if a person can move one space in 1 minute, or move from position x to 2x in 2 minutes
// find the fastest route to the N-th tile
// 2/5/2024
use std::io;
use std::cmp; 

fn main() {
    let mut input = String::new();
    println!("Number of tiles:");

    let _ = io::stdin().read_line(&mut input);
    let num_tiles: usize = input.trim().parse().expect("Please enter number"); 

    // shift by so first start has index 1 since 0 * 2 = 0 (breaks tram)
    let mut tiles: Vec<i32> = vec![0; num_tiles + 1];

    for i in (1..num_tiles).rev() {
        let mut tram_cost: i32 = i32::MAX;
        let mut walk_cost: i32 = i32::MAX;

        if i * 2 < tiles.len() {
            tram_cost = tiles[i * 2] + 2;
        }
        if i + 1 < tiles.len() {
            walk_cost = tiles[i + 1] + 1;
        }
        tiles[i] = cmp::min(tram_cost, walk_cost);
    }
    println!("Minimum travel time: {}", tiles[1]);
}
