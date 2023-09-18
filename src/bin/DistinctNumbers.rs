// https://cses.fi/problemset/task/1621

use std::collections::HashSet;

fn main() {
    let mut inp: String = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Error in input");
    let n: u64 = inp.trim().parse().expect("Error in parsing to int");
    inp.clear();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Error in input");
    let x: Vec<u64> = inp
        .split_whitespace()
        .map(|s| s.parse().expect("Error in input"))
        .collect();
    let mut map: HashSet<u64> = HashSet::new();
    // Use a for loop to insert values into the HashSet
    for key in &x {
        map.insert(*key);
    }
    println!("{}", map.len());
}
